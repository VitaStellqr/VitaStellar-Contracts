#!/usr/bin/env bash
# Security scan wrapper for dependency, unsafe-code, and custom lint checks.
#
# Custom lint intentionally flags plain http:// references, except local
# development endpoints: http://localhost, http://127.0.0.1, and http://0.0.0.0.
set -euo pipefail

REPORT_DIR="${1:-security-reports}"
mkdir -p "$REPORT_DIR"

AUDIT_JSON="$REPORT_DIR/cargo-audit.json"
GEIGER_OUT="$REPORT_DIR/cargo-geiger.txt"
CUSTOM_OUT="$REPORT_DIR/custom-security-lint.txt"
SUMMARY_MD="$REPORT_DIR/security-summary.md"

critical=0
medium=0

CUSTOM_LINT_PATTERN="(api[_-]?key|secret[_-]?key|private[_-]?key|BEGIN (RSA|EC|OPENSSH) PRIVATE KEY|password[[:space:]]*=[[:space:]]*['\\\"][^'\\\"]+['\\\"]|http://)"
LOCAL_HTTP_WHITELIST_PATTERN="http://(localhost|127\.0\.0\.1|0\.0\.0\.0)([:/]|$)"

run_custom_security_lint() {
  grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=security-reports \
    "$CUSTOM_LINT_PATTERN" \
    . | grep -Ev "$LOCAL_HTTP_WHITELIST_PATTERN" || true
}

if [ "${SECURITY_SCAN_SELF_CHECK:-}" = "1" ]; then
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT

  mkdir -p "$tmpdir/repo"
  {
    printf '%s\n' "external endpoint: http"'://example.com/api'
    printf '%s\n' "localhost endpoint: http"'://localhost:8000/soroban/rpc'
    printf '%s\n' "loopback endpoint: http"'://127.0.0.1:8000/soroban/rpc'
    printf '%s\n' "bind endpoint: http"'://0.0.0.0:8000/soroban/rpc'
  } > "$tmpdir/repo/sample.txt"

  findings="$(
    cd "$tmpdir/repo"
    run_custom_security_lint
  )"

  if ! echo "$findings" | grep -q "http"'://example.com/api'; then
    echo "self-check failed: external http:// reference was not flagged" >&2
    exit 1
  fi

  if echo "$findings" | grep -Eq "http://(localhost|127\.0\.0\.1|0\.0\.0\.0)"; then
    echo "self-check failed: local http:// reference was flagged" >&2
    echo "$findings" >&2
    exit 1
  fi

  echo "security-scan self-check passed"
  exit 0
fi

echo "Running cargo-audit..."
if ! cargo audit --json > "$AUDIT_JSON"; then
  # cargo-audit exits non-zero when advisories are found; severity-based gating is handled below.
  true
fi

audit_counts="$(python3 - <<'PY' "$AUDIT_JSON"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as fh:
    data = json.load(fh)
vulns = data.get("vulnerabilities", {}).get("list", [])
critical = 0
medium = 0
for vuln in vulns:
    advisory = vuln.get("advisory", {})
    sev = str(advisory.get("severity", "")).lower()
    if sev in {"critical", "high"}:
        critical += 1
    elif sev in {"medium", "moderate"}:
        medium += 1
print(f"{critical} {medium} {len(vulns)}")
PY
)"

audit_critical="$(echo "$audit_counts" | awk '{print $1}')"
audit_medium="$(echo "$audit_counts" | awk '{print $2}')"
audit_total="$(echo "$audit_counts" | awk '{print $3}')"

critical=$((critical + audit_critical))
medium=$((medium + audit_medium))

echo "Running cargo-geiger..."
if cargo geiger --workspace --all-features --all-targets > "$GEIGER_OUT" 2>&1; then
  unsafe_total="$(grep -Eo "[0-9]+ unsafe" "$GEIGER_OUT" | awk '{sum+=$1} END {print sum+0}')"
else
  echo "cargo-geiger execution failed; treating as non-blocking informational warning." >> "$GEIGER_OUT"
  unsafe_total="n/a"
fi

echo "Running custom security lint rules..."
{
  echo "Potential hardcoded secrets and weak patterns"
  run_custom_security_lint
} > "$CUSTOM_OUT"

custom_findings="$(grep -c ":" "$CUSTOM_OUT" || true)"
if [ "$custom_findings" -gt 0 ]; then
  medium=$((medium + custom_findings))
fi

{
  echo "## Security Scan Report"
  echo
  echo "- Cargo audit advisories: $audit_total"
  echo "- Critical/High advisories: $audit_critical"
  echo "- Medium advisories: $audit_medium"
  echo "- Unsafe usages reported by cargo-geiger: $unsafe_total"
  echo "- Custom lint findings: $custom_findings"
  echo
  if [ "$critical" -gt 0 ]; then
    echo "❌ Blocking: critical/high security findings detected."
  elif [ "$medium" -gt 0 ]; then
    echo "⚠️ Warning: medium security findings detected."
  else
    echo "✅ No blocking security findings detected."
  fi
} > "$SUMMARY_MD"

cat "$SUMMARY_MD"

if [ "$critical" -gt 0 ]; then
  exit 1
fi

exit 0
