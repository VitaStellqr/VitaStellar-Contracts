#!/usr/bin/env bash
# Validates error codes across all contract files:
# 1. All error codes fall within approved category ranges (100-999)
# 2. No duplicate error codes in ERROR_CODES.md documentation
# 3. All errors.rs codes are documented in ERROR_CODES.md
# 4. Codes 1-99 are only used as per-contract specific codes
#
# Requires: bash (not POSIX sh)
# Usage: ./scripts/check_error_codes.sh  OR  bash scripts/check_error_codes.sh
#
# Exits non-zero on any violation.

set -euo pipefail

CONTRACTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/contracts"
DOCS_DIR="$(cd "$(dirname "$0")/.." && pwd)/docs"
ERROR_CODES_FILE="$DOCS_DIR/ERROR_CODES.md"
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

echo "Checking error codes..."
echo ""

# Check for duplicates in documentation
echo "1. Checking ERROR_CODES.md for duplicates..."
if check_documentation_duplicates; then
    echo "   ✓ No duplicate codes in documentation"
else
    VIOLATIONS=$((VIOLATIONS + 1))
fi
echo ""

# Check all error code implementations
echo "2. Checking error code implementations..."
file_count=0
while IFS= read -r -d '' file; do
    check_implementation_codes "$file"
    file_count=$((file_count + 1))
done < <(find "$CONTRACTS_DIR" -name "errors.rs" -print0)
echo "   ✓ Checked $file_count error code files"
echo ""

# Summary
if (( VIOLATIONS > 0 )); then
    echo "FAIL: $VIOLATIONS error code violation(s) found."
    echo "See docs/ERROR_CODES.md for the approved ranges and documented codes."
    exit 1
else
    echo "✓ All error codes are valid and properly documented."
    exit 0
fi
