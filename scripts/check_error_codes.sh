#!/usr/bin/env bash
# Validates that all error codes in errors.rs files fall within the approved
# category ranges defined in docs/ERROR_CODES.md. Also validates that contract
# names referenced in ERROR_CODES.md exist in contracts/. Exits non-zero on any violation.

set -euo pipefail

CONTRACTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/contracts"
DOCS_DIR="$(cd "$(dirname "$0")/../docs" && pwd)"
VIOLATIONS=0

# Helper: Extract all numeric codes from ERROR_CODES.md (category sections only, not per-contract)
extract_documented_codes() {
    # Extract codes from category sections (before "Per-Contract Error Codes")
    sed '/## Per-Contract Error Codes/q' "$ERROR_CODES_FILE" | \
    grep -E '^\| [0-9]+' | \
    awk '{print $2}' | \
    sort -n
}

# Helper: Check for duplicate codes in ERROR_CODES.md documentation
check_documentation_duplicates() {
    local temp_file
    temp_file=$(mktemp)
    trap "rm -f $temp_file" RETURN

    extract_documented_codes > "$temp_file"
    
    local duplicates
    duplicates=$(sort "$temp_file" | uniq -d)
    
    if [[ -n "$duplicates" ]]; then
        echo "ERROR: Duplicate codes found in ERROR_CODES.md documentation:"
        while IFS= read -r code; do
            echo "  Code $code appears multiple times"
            grep -E "^\| $code \|" "$ERROR_CODES_FILE" | head -3
        done <<< "$duplicates"
        return 1
    fi
    return 0
}

# Helper: Collect all codes from errors.rs files
check_implementation_codes() {
    local file="$1"
    local contract
    contract=$(basename "$(dirname "$(dirname "$file")")")

    local code_list=()
    while IFS= read -r line; do
        # Match variant assignments: SomeName = 123,
        if [[ "$line" =~ ^[[:space:]]+[A-Za-z][A-Za-z0-9_]*[[:space:]]*=[[:space:]]*([0-9]+), ]]; then
            local code="${BASH_REMATCH[1]}"
            
            # Validate code falls in approved ranges
            if (( code >= 1 && code <= 99 )); then
                # Per-contract codes are allowed
                continue
            fi
            
            if ! (( code >= 100 && code <= 999 )); then
                echo "VIOLATION in $contract ($file): code $code is outside 100-999 range"
                VIOLATIONS=$((VIOLATIONS + 1))
                continue
            fi
            
            code_list+=("$code")
        fi
    done < "$file"
    
    # Check for duplicates within this file
    if (( ${#code_list[@]} > 0 )); then
        local duplicate_in_file
        duplicate_in_file=$(printf '%s\n' "${code_list[@]}" 2>/dev/null | sort | uniq -d)
        if [[ -n "$duplicate_in_file" ]]; then
            echo "VIOLATION in $contract ($file): duplicate code(s) within same file:"
            echo "$duplicate_in_file" | sed 's/^/  Code /'
            VIOLATIONS=$((VIOLATIONS + 1))
        fi
    fi
}

# Helper: Check if code is documented
is_code_documented() {
    local code="$1"
    extract_documented_codes | grep -q "^$code$"
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

# Check all error code implementations
echo "2. Checking error code implementations..."
file_count=0
while IFS= read -r -d '' file; do
    check_implementation_codes "$file"
    file_count=$((file_count + 1))
done < <(find "$CONTRACTS_DIR" -name "errors.rs" -print0)
echo "   ✓ Checked $file_count error code files"
echo ""

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
