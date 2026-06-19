#!/usr/bin/env bash

# Healthcare Compliance Verification Suite
# Runs real build and cargo test coverage for the healthcare-related contracts
# that back the compliance workflows in this repository.

set -uo pipefail

# Colors for output
# shellcheck disable=SC2034 # Color variables are used in echo -e statements.
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
export CARGO_NET_OFFLINE=true

BUILD_TARGETS=()
TEST_SECTIONS=()
SECTION_PACKAGES=()
SECTION_COMMANDS=()
SECTION_STATUSES=()
SECTION_TEST_COUNTS=()

TOTAL_BUILD_STEPS=0
PASSED_BUILD_STEPS=0
TOTAL_TESTS=0
FAILED_STEPS=0

print_status() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC}: $message"
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}✗ FAIL${NC}: $message"
    elif [ "$status" = "SKIP" ]; then
        echo -e "${YELLOW}○ SKIP${NC}: $message"
    else
        echo -e "${BLUE}● INFO${NC}: $message"
    fi
}

extract_passed_tests() {
    local output_file=$1
    local passed
    passed=$(sed -n 's/.*test result: ok\. \([0-9][0-9]*\) passed.*/\1/p' "$output_file" | tail -n 1)
    if [ -n "$passed" ]; then
        printf '%s' "$passed"
    else
        printf '0'
    fi
}

run_cargo_command() {
    local label=$1
    shift

    echo -e "\n${BLUE}${label}${NC}"
    echo "----------------------------------------"

    local output_file
    output_file=$(mktemp)

    (
        cd "$PROJECT_ROOT" || exit 1
        "$@"
    ) 2>&1 | tee "$output_file"

    local status=${PIPESTATUS[0]}
    local passed_tests=0

    if [ "$status" -eq 0 ]; then
        passed_tests=$(extract_passed_tests "$output_file")
        if [ "$passed_tests" -gt 0 ]; then
            print_status "PASS" "$label ($passed_tests tests)"
        else
            print_status "PASS" "$label"
        fi
    else
        print_status "FAIL" "$label"
        FAILED_STEPS=$((FAILED_STEPS + 1))
    fi

    rm -f "$output_file"

    RUN_PASSED_TESTS=$passed_tests
    return "$status"
}

run_build() {
    local package=$1
    local label=$2

    TOTAL_BUILD_STEPS=$((TOTAL_BUILD_STEPS + 1))
    if run_cargo_command "$label" cargo +stable build --release -p "$package"; then
        PASSED_BUILD_STEPS=$((PASSED_BUILD_STEPS + 1))
        BUILD_TARGETS+=("$package")
    fi
}

run_test_suite() {
    local section_name=$1
    local package=$2
    local label=$3
    shift 3

    local -a cmd=(cargo +stable test --release -p "$package")
    if [ "$#" -gt 0 ]; then
        cmd+=("$@")
    fi
    cmd+=(-- --nocapture)

    if run_cargo_command "$label" "${cmd[@]}"; then
        TOTAL_TESTS=$((TOTAL_TESTS + RUN_PASSED_TESTS))
        TEST_SECTIONS+=("$section_name")
        SECTION_PACKAGES+=("$package")
        SECTION_COMMANDS+=("$label")
        SECTION_STATUSES+=("PASS")
        SECTION_TEST_COUNTS+=("$RUN_PASSED_TESTS")
    else
        TEST_SECTIONS+=("$section_name")
        SECTION_PACKAGES+=("$package")
        SECTION_COMMANDS+=("$label")
        SECTION_STATUSES+=("FAIL")
        SECTION_TEST_COUNTS+=("0")
    fi
}

test_hipaa_compliance() {
    echo -e "\n${BLUE}HIPAA Compliance Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "HIPAA compliance" \
        "cross_chain_access" \
        "Cross-chain access and emergency coverage" \
        test_
}

test_gdpr_compliance() {
    echo -e "\n${BLUE}GDPR Compliance Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "GDPR compliance" \
        "medical_consent_nft" \
        "Consent lifecycle and revocation coverage" \
        test_
}

test_fhir_integration() {
    echo -e "\n${BLUE}FHIR Integration Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "FHIR integration" \
        "healthcare_data_conversion" \
        "FHIR and healthcare format conversion coverage" \
        test_
}

test_audit_trail() {
    echo -e "\n${BLUE}Audit Trail Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "Audit trail" \
        "audit_forensics" \
        "Audit forensics and compliance reporting coverage" \
        test_
}

test_breach_management() {
    echo -e "\n${BLUE}Breach Management Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "Breach management" \
        "notification_system" \
        "Breach notification workflow coverage" \
        test_trigger_
}

test_compliance_dashboard() {
    echo -e "\n${BLUE}Compliance Dashboard Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "Compliance dashboard" \
        "notification_system" \
        "Compliance analytics coverage" \
        test_analytics_
}

test_integration() {
    echo -e "\n${BLUE}Integration Coverage${NC}"
    echo "========================================"
    run_test_suite \
        "Integration" \
        "emr_integration" \
        "EMR interoperability and conversion coverage" \
        test_
}

test_performance() {
    echo -e "\n${BLUE}Performance Coverage${NC}"
    echo "========================================"

    run_test_suite \
        "Performance" \
        "notification_system" \
        "Notification throughput coverage" \
        test_rate_limit

    run_test_suite \
        "Performance" \
        "notification_system" \
        "Bulk notification coverage" \
        test_bulk_
}

generate_compliance_report() {
    echo -e "\n${BLUE}Generating Compliance Report${NC}"
    echo "========================================"

    local report_file="$PROJECT_ROOT/compliance_test_report_$(date +%Y%m%d_%H%M%S).md"

    {
        echo "# Healthcare Compliance Verification Report"
        echo
        echo "**Generated:** $(date -u +%Y-%m-%dT%H:%M:%SZ)"
        echo "**Scope:** Real cargo build and cargo test coverage for healthcare-related contracts"
        echo
        echo "## Build Summary"
        echo
        echo "- Build steps passed: $PASSED_BUILD_STEPS/$TOTAL_BUILD_STEPS"
        echo
        echo "## Test Summary"
        echo
        echo "- Total real tests passed: $TOTAL_TESTS"
        echo
        echo "## Section Results"
        echo
        printf '| Section | Package | Coverage Label | Tests Passed | Status |\n'
        printf '| --- | --- | --- | ---: | --- |\n'
        local i
        for i in "${!TEST_SECTIONS[@]}"; do
            printf '| %s | %s | %s | %s | %s |\n' \
                "${TEST_SECTIONS[$i]}" \
                "${SECTION_PACKAGES[$i]}" \
                "${SECTION_COMMANDS[$i]}" \
                "${SECTION_TEST_COUNTS[$i]}" \
                "${SECTION_STATUSES[$i]}"
        done
        echo
        echo "## Build Targets"
        echo
        if [ "${#BUILD_TARGETS[@]}" -eq 0 ]; then
            echo "- None recorded"
        else
            local build_target
            for build_target in "${BUILD_TARGETS[@]}"; do
                echo "- $build_target"
            done
        fi
        echo
        echo "## Notes"
        echo
        echo "- The repository does not contain a contracts/healthcare_compliance crate."
        echo "- This suite now validates the actual compliance-related contract packages present in the workspace."
        echo "- Reported counts come from cargo's own test summaries."
    } > "$report_file"

    print_status "PASS" "Compliance report generated: $report_file"
    cat "$report_file"
}

main() {
    run_build "regulatory_compliance" "Building regulatory_compliance"
    run_build "cross_chain_access" "Building cross_chain_access"
    run_build "medical_consent_nft" "Building medical_consent_nft"
    run_build "healthcare_data_conversion" "Building healthcare_data_conversion"
    run_build "audit_forensics" "Building audit_forensics"
    run_build "notification_system" "Building notification_system"
    run_build "emr_integration" "Building emr_integration"

    test_hipaa_compliance
    test_gdpr_compliance
    test_fhir_integration
    test_audit_trail
    test_breach_management
    test_compliance_dashboard
    test_integration
    test_performance

    generate_compliance_report

    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}  Test Summary${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo -e "Build steps: $PASSED_BUILD_STEPS/$TOTAL_BUILD_STEPS passed"
    echo -e "Real tests passed: $TOTAL_TESTS"
    echo -e "Failed steps: $FAILED_STEPS"

    if [ "$FAILED_STEPS" -eq 0 ]; then
        echo -e "\n${GREEN}All compliance verification steps passed.${NC}"
        return 0
    fi

    echo -e "\n${RED}Some compliance verification steps failed. Review the output above.${NC}"
    return 1
}

main "$@"
