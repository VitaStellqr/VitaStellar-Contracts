#!/usr/bin/env bash
# Validates that all error codes in errors.rs files fall within the approved
# category ranges defined in docs/ERROR_CODES.md. Also validates that contract
# names referenced in ERROR_CODES.md exist in contracts/. Exits non-zero on any violation.

set -euo pipefail

CONTRACTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/contracts"
DOCS_DIR="$(cd "$(dirname "$0")/../docs" && pwd)"
VIOLATIONS=0

check_file() {
    local file="$1"
    local contract
    contract=$(basename "$(dirname "$(dirname "$file")")")

    while IFS= read -r line; do
        # Match variant assignments: SomeName = 123,
        if [[ "$line" =~ ^[[:space:]]+[A-Za-z][A-Za-z0-9_]*[[:space:]]*=[[:space:]]*([0-9]+), ]]; then
            code="${BASH_REMATCH[1]}"
            # Validate code falls in one of the approved ranges
            if ! (( (code >= 100 && code <= 999) )); then
                echo "VIOLATION in $contract ($file): code $code is outside 100-999 range"
                VIOLATIONS=$((VIOLATIONS + 1))
            fi
            # Check for legacy sequential codes (1-99) which indicate not-yet-migrated files
            if (( code >= 1 && code <= 99 )); then
                echo "VIOLATION in $contract ($file): code $code uses legacy sequential numbering (expected 100+)"
                VIOLATIONS=$((VIOLATIONS + 1))
            fi
        fi
    done < "$file"
}

validate_contract_names() {
    local error_codes_file="$DOCS_DIR/ERROR_CODES.md"
    local actual_contracts
    local referenced_contracts
    
    # Get list of actual contracts
    actual_contracts=$(find "$CONTRACTS_DIR" -maxdepth 1 -type d ! -name contracts -printf '%f\n' | sort)
    
    # Extract contract names only from error code tables (rows with at least 6 columns and numeric code)
    # Pattern: | numeric | symbol | contracts | description | causes | remediation |
    referenced_contracts=$(awk -F'|' '
        /^## Per-Contract Error Codes/ { exit }
        /^\| [0-9]{2,}/ {
            # Count fields to ensure this is an error code table row
            if (NF >= 12) {  # 6 fields with pipes = 12 total including empty ones
                contracts=$4  # 4th column is Contract(s)
                gsub(/^[[:space:]]+/, "", contracts)  # trim leading spaces
                gsub(/[[:space:]]+$/, "", contracts)  # trim trailing spaces
                if (contracts && contracts != "Contract(s)") {
                    # split by comma and print each contract
                    n=split(contracts, arr, ",")
                    for (i=1; i<=n; i++) {
                        contract=arr[i]
                        gsub(/^[[:space:]]+/, "", contract)
                        gsub(/[[:space:]]+$/, "", contract)
                        if (contract && contract != "All") {
                            print contract
                        }
                    }
                }
            }
        }
    ' "$error_codes_file" | sort -u)
    
    # Check each referenced contract exists
    while IFS= read -r contract; do
        if [[ -z "$contract" ]]; then
            continue
        fi
        if ! echo "$actual_contracts" | grep -q "^$contract$"; then
            echo "VIOLATION in ERROR_CODES.md: contract '$contract' does not exist in contracts/"
            VIOLATIONS=$((VIOLATIONS + 1))
        fi
    done <<< "$referenced_contracts"
}

echo "Checking error codes across contracts..."

while IFS= read -r -d '' file; do
    check_file "$file"
done < <(find "$CONTRACTS_DIR" -name "errors.rs" -print0)

echo "Validating contract names in ERROR_CODES.md..."
validate_contract_names

if (( VIOLATIONS > 0 )); then
    echo ""
    echo "FAIL: $VIOLATIONS violation(s) found."
    echo "See docs/ERROR_CODES.md for the approved ranges and contract list."
    exit 1
else
    echo "OK: all error codes and contract references are valid."
    exit 0
fi
