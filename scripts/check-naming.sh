#!/bin/bash
# Naming Convention Check Script for VitaStellar Contracts
# This script helps identify naming inconsistencies across the codebase

echo "🔍 Checking naming conventions across VitaStellar Contracts..."
echo "======================================================"

VIOLATIONS=0

# Check for non-snake_case function names
echo ""
echo "1. Checking function names (should be snake_case)..."
echo "--------------------------------------------------"
FN_VIOLATIONS=$(find contracts -name "*.rs" -type f -exec grep -H "pub fn [A-Z]" {} \; | head -20)
if [ -n "$FN_VIOLATIONS" ]; then
    echo "$FN_VIOLATIONS"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo "✓ No violations found"
fi

# Check for non-SCREAMING_SNAKE_CASE constants
echo ""
echo "2. Checking constant names (should be SCREAMING_SNAKE_CASE)..."
echo "------------------------------------------------------------"
CONST_VIOLATIONS=$(find contracts -name "*.rs" -type f -exec grep -H "const [a-z]" {} \; | head -20)
if [ -n "$CONST_VIOLATIONS" ]; then
    echo "$CONST_VIOLATIONS"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo "✓ No violations found"
fi

# Check for non-PascalCase type definitions
echo ""
echo "3. Checking type names (should be PascalCase)..."
echo "------------------------------------------------"
STRUCT_VIOLATIONS=$(find contracts -name "*.rs" -type f -exec grep -H "struct [a-z]" {} \; | head -10)
ENUM_VIOLATIONS=$(find contracts -name "*.rs" -type f -exec grep -H "enum [a-z]" {} \; | head -10)
if [ -n "$STRUCT_VIOLATIONS" ] || [ -n "$ENUM_VIOLATIONS" ]; then
    echo "$STRUCT_VIOLATIONS"
    echo "$ENUM_VIOLATIONS"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo "✓ No violations found"
fi

# Check for Err instead of Error
echo ""
echo "4. Checking for 'Err' instead of 'Error'..."
echo "------------------------------------------"
ERR_VIOLATIONS=$(find contracts -name "*.rs" -type f -exec grep -H "enum Err" {} \;)
if [ -n "$ERR_VIOLATIONS" ]; then
    echo "$ERR_VIOLATIONS"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo "✓ No violations found"
fi

# Check module names
echo ""
echo "5. Checking module names (should be snake_case)..."
echo "-------------------------------------------------"
MODULE_NAMES=$(find contracts -name "mod.rs" -type f -exec dirname {} \; | xargs -I {} basename {} | sort | uniq)
echo "$MODULE_NAMES"

echo ""
echo "======================================================"
if [ $VIOLATIONS -gt 0 ]; then
    echo "❌ Naming check failed with $VIOLATIONS violation(s)!"
    echo ""
    echo "To fix issues, refer to:"
    echo "  - docs/CODING_STANDARDS.md for naming conventions"
    echo "  - .clippy.toml for linting rules"
    echo ""
    echo "Run 'cargo clippy -- -D warnings' for detailed linting."
    exit 1
else
    echo "✅ Naming check complete! No violations found."
    exit 0
fi