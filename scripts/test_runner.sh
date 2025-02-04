#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
COVERAGE_THRESHOLD=80
GAS_THRESHOLD=50000
TEST_TIMEOUT=300

# Initialize counters
TOTAL_TESTS=0
FAILED_TESTS=0

# Create results directory with timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="target/test_results_${TIMESTAMP}"
mkdir -p "${RESULTS_DIR}"

# Logging function
log() {
    echo -e "${2:-$NC}$1${NC}"
    echo "$1" >> "${RESULTS_DIR}/test.log"
}

# Run test with timeout
run_test() {
    local test_name=$1
    local test_cmd=$2
    local timeout_sec=$3
    
    log "Running test: ${test_name}" "${YELLOW}"
    
    timeout ${timeout_sec} ${test_cmd} > "${RESULTS_DIR}/${test_name}.log" 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        log "✓ ${test_name} passed" "${GREEN}"
        return 0
    elif [ $exit_code -eq 124 ]; then
        log "✗ ${test_name} timed out after ${timeout_sec} seconds" "${RED}"
        return 1
    else
        log "✗ ${test_name} failed with exit code ${exit_code}" "${RED}"
        return 1
    fi
}

# Run unit tests
log "Running unit tests..." "${YELLOW}"
if run_test "unit_tests" "cargo test --lib" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Run property-based tests
log "Running property-based tests..." "${YELLOW}"
if run_test "property_tests" "cargo test --test property" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Run formal verification
log "Running formal verification..." "${YELLOW}"
if run_test "verification" "cargo verify" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Run gas tests
log "Running gas tests..." "${YELLOW}"
if run_test "gas_tests" "cargo test --test gas" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Run integration tests
log "Running integration tests..." "${YELLOW}"
if run_test "integration_tests" "cargo test --test integration" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Run privacy tests
log "Running privacy tests..." "${YELLOW}"
if run_test "privacy_tests" "cargo test --test privacy" $TEST_TIMEOUT; then
    ((TOTAL_TESTS++))
else
    ((FAILED_TESTS++))
    ((TOTAL_TESTS++))
fi

# Generate coverage report
log "Generating coverage report..." "${YELLOW}"
cargo coverage
coverage_percent=$(grep -oP '(?<=Total coverage: )\d+' "${RESULTS_DIR}/coverage.txt")

# Generate HTML report
log "Generating HTML test report..." "${YELLOW}"
cat > "${RESULTS_DIR}/report.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>Stremax Test Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .summary { margin-bottom: 20px; }
        .pass { color: green; }
        .fail { color: red; }
    </style>
</head>
<body>
    <h1>Test Results - ${TIMESTAMP}</h1>
    <div class="summary">
        <p>Total Tests: ${TOTAL_TESTS}</p>
        <p>Failed Tests: ${FAILED_TESTS}</p>
        <p>Coverage: ${coverage_percent}%</p>
    </div>
    <h2>Test Logs</h2>
    <pre>
$(cat "${RESULTS_DIR}/test.log")
    </pre>
</body>
</html>
EOF

# Print summary
log "\nTest Summary:" "${YELLOW}"
log "Total Tests: ${TOTAL_TESTS}"
log "Failed Tests: ${FAILED_TESTS}"
log "Coverage: ${coverage_percent}%"

# Check if tests passed
if [ $FAILED_TESTS -eq 0 ] && [ $coverage_percent -ge $COVERAGE_THRESHOLD ]; then
    log "\nAll tests passed successfully!" "${GREEN}"
    exit 0
else
    log "\nTests failed or coverage below threshold!" "${RED}"
    exit 1
fi 