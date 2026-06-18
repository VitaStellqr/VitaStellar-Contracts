#!/usr/bin/env bash
# loc_report.sh — report or validate source-only lines of code for every contract.
#
# Counts lines in `contracts/<name>/src/**/*.rs` (tests are excluded because they
# are not part of the deployed attack surface). The same counts back the
# `loc-manifest` block in `docs/MASTER_THREAT_MODEL.md`, so the script doubles as
# both a generator and a CI gate.
#
# Usage:
#   ./scripts/loc_report.sh                 # emit a markdown table
#   ./scripts/loc_report.sh --emit-manifest # emit a `name: loc` block ready to paste
#   ./scripts/loc_report.sh --contract NAME # print LOC for a single contract
#   ./scripts/loc_report.sh --check         # validate the manifest in MASTER_THREAT_MODEL.md
#   ./scripts/loc_report.sh -h | --help     # show this message
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONTRACTS_DIR="contracts"
DOC_FILE="docs/MASTER_THREAT_MODEL.md"

# Count LOC for a single contract directory (src/**/*.rs only).
# Returns "0" if the directory has no src/ subdir. Output is a single integer.
contract_loc() {
    local dir="$1/src"
    if [ ! -d "$dir" ]; then
        printf '0\n'
        return 0
    fi
    # `find … -exec cat +` is portable to both GNU and BSD find, so this works
    # identically on Linux and macOS.
    find "$dir" -type f -name '*.rs' -exec cat {} + 2>/dev/null \
        | wc -l \
        | tr -d ' '
}

# Emit the contract directory names in stable alphabetical order. Sorting in
# one place keeps the output deterministic across machines and CI runners.
list_contracts() {
    local d
    for d in "$CONTRACTS_DIR"/*/; do
        [ -d "$d" ] || continue
        basename "$d"
    done | sort
}

# Pull the contents of the fenced ```loc-manifest block out of the doc.
# Only the *first* such block is honored; if a contributor adds a second one by
# accident the earlier one takes precedence (and CI will surface drift).
extract_manifest() {
    awk '
        /^```loc-manifest$/ { capture = 1; next }
        /^```$/             { if (capture) { capture = 0; exit } }
        capture             { print }
    ' "$DOC_FILE"
}

# Emit the same `name: loc` lines as the embedded manifest but with current
# actual counts. Skips blank lines and `#` comments. If there is no manifest in
# the doc yet, fall back to every contract under contracts/ so the user has a
# starting block to paste in.
emit_current_manifest() {
    local manifest="$1"
    if [ -z "$manifest" ]; then
        local name
        for name in $(list_contracts); do
            printf '%s: %s\n' "$name" "$(contract_loc "$CONTRACTS_DIR/$name")"
        done | sort
        return 0
    fi
    local line name
    while IFS= read -r line; do
        line="${line%$'\r'}"
        [ -z "$line" ] && continue
        case "$line" in \#*) continue ;; esac
        if [[ "$line" =~ ^([a-zA-Z0-9_-]+):[[:space:]]*([0-9]+)$ ]]; then
            name="${BASH_REMATCH[1]}"
            printf '%s: %s\n' "$name" "$(contract_loc "$CONTRACTS_DIR/$name")"
        fi
    done <<< "$manifest" | sort
}

cmd_report() {
    echo "| Contract | LOC |"
    echo "|---|---|"
    local name loc
    for name in $(list_contracts); do
        loc="$(contract_loc "$CONTRACTS_DIR/$name")"
        printf '| %s | %s |\n' "$name" "$loc"
    done
}

cmd_emit_manifest() {
    local manifest
    manifest="$(extract_manifest)"
    emit_current_manifest "$manifest"
}

cmd_contract() {
    local name="${1:-}"
    if [ -z "$name" ]; then
        echo "error: --contract requires a contract name" >&2
        exit 2
    fi
    if [ ! -d "$CONTRACTS_DIR/$name" ]; then
        echo "error: contract '$name' not found under $CONTRACTS_DIR/" >&2
        exit 2
    fi
    contract_loc "$CONTRACTS_DIR/$name"
}

cmd_check() {
    if [ ! -f "$DOC_FILE" ]; then
        echo "error: $DOC_FILE not found" >&2
        exit 2
    fi

    local manifest
    manifest="$(extract_manifest)"

    if [ -z "$manifest" ]; then
        echo "error: no \`loc-manifest\` block found in $DOC_FILE" >&2
        echo "       run '$0 --emit-manifest' and paste the output into a fenced" >&2
        echo "       \`\`\`loc-manifest\` block to enable validation." >&2
        exit 1
    fi

    local failures=0
    local checked=0
    local line name declared actual
    while IFS= read -r line; do
        line="${line%$'\r'}"
        [ -z "$line" ] && continue
        case "$line" in \#*) continue ;; esac
        if ! [[ "$line" =~ ^([a-zA-Z0-9_-]+):[[:space:]]*([0-9]+)$ ]]; then
            echo "  ✗ malformed manifest line: $line" >&2
            failures=$((failures + 1))
            continue
        fi
        name="${BASH_REMATCH[1]}"
        declared="${BASH_REMATCH[2]}"
        checked=$((checked + 1))

        if [ ! -d "$CONTRACTS_DIR/$name" ]; then
            echo "  ✗ $name: declared $declared but contracts/$name/ does not exist (phantom contract reference)" >&2
            failures=$((failures + 1))
            continue
        fi

        actual="$(contract_loc "$CONTRACTS_DIR/$name")"
        if [ "$actual" != "$declared" ]; then
            echo "  ✗ $name: declared=$declared, actual=$actual (drift: $((actual - declared)) lines)" >&2
            failures=$((failures + 1))
        else
            echo "  ✓ $name: $actual lines"
        fi
    done <<< "$manifest"

    echo
    echo "Validated $checked contracts against $DOC_FILE: $((checked - failures)) ok, $failures mismatched."
    if [ "$failures" -gt 0 ]; then
        echo
        echo "─── corrected manifest (paste over the loc-manifest block) ───"
        emit_current_manifest "$manifest" | sed 's/^/  /'
        echo "────────────────────────────────────────────────────────────────"
        echo >&2
        echo "Run '$0 --emit-manifest' for a clean machine-readable copy." >&2
        exit 1
    fi
}

usage() {
    cat <<EOF
loc_report.sh — emit or validate contract LOC counts.

Usage:
  $0                  print a markdown table of every contract's LOC
  $0 --emit-manifest  print a 'name: loc' block ready to paste into MASTER_THREAT_MODEL.md
  $0 --contract NAME  print the LOC count for a single contract
  $0 --check          validate the loc-manifest block in docs/MASTER_THREAT_MODEL.md
  $0 -h | --help      show this message
EOF
}

main() {
    case "${1:-}" in
        "")
            cmd_report
            ;;
        --emit-manifest)
            cmd_emit_manifest
            ;;
        --check)
            cmd_check
            ;;
        --contract)
            shift
            cmd_contract "${1:-}"
            ;;
        -h | --help)
            usage
            ;;
        *)
            echo "error: unknown flag '$1'" >&2
            usage >&2
            exit 2
            ;;
    esac
}

main "$@"
