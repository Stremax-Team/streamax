#!/bin/bash

# Stremax Test Runner
# This script runs all test suites and generates reports

set -e

# Configuration
COVERAGE_THRESHOLD=90
GAS_THRESHOLD=1000000
TEST_TIMEOUT=300

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Print header
echo -e "${YELLOW}Stremax Test Runner${NC}"
echo "================================"

# Function to run tests with timeout
run_with_timeout() {
    timeout $TEST_TIMEOUT $@
    return $?
}

# Function to check test results
check_result() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Passed${NC}"
    else
        echo -e "${RED}✗ Failed${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

# Initialize counters
FAILED_TESTS=0
TOTAL_TESTS=0

# Create test results directory
RESULTS_DIR="test-results-$(date +%Y%m%d-%H%M%S)"
mkdir -p $RESULTS_DIR

echo "Running tests..."
echo "----------------"

# 1. Unit Tests
echo -n "Running unit tests... "
run_with_timeout strm test --unit > $RESULTS_DIR/unit-tests.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 2. Property-Based Tests
echo -n "Running property tests... "
run_with_timeout strm test --property > $RESULTS_DIR/property-tests.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 3. Integration Tests
echo -n "Running integration tests... "
run_with_timeout strm test --integration > $RESULTS_DIR/integration-tests.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 4. Formal Verification
echo -n "Running verification tests... "
run_with_timeout strm verify > $RESULTS_DIR/verification.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 5. Gas Tests
echo -n "Running gas optimization tests... "
run_with_timeout strm test --gas > $RESULTS_DIR/gas-tests.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 6. Privacy Tests
echo -n "Running privacy tests... "
run_with_timeout strm test --privacy > $RESULTS_DIR/privacy-tests.log
check_result
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# 7. Coverage Analysis
echo -n "Running coverage analysis... "
strm coverage > $RESULTS_DIR/coverage.log
COVERAGE=$(grep "Total coverage:" $RESULTS_DIR/coverage.log | awk '{print $3}' | sed 's/%//')
if [ $(echo "$COVERAGE < $COVERAGE_THRESHOLD" | bc -l) -eq 1 ]; then
    echo -e "${RED}✗ Failed - Coverage $COVERAGE% below threshold${NC}"
    FAILED_TESTS=$((FAILED_TESTS + 1))
else
    echo -e "${GREEN}✓ Passed - Coverage $COVERAGE%${NC}"
fi
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# Generate HTML report
echo "Generating test report..."
cat > $RESULTS_DIR/report.html << EOF
<!DOCTYPE html>
<html>
<head>
    <title>Stremax Test Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .passed { color: green; }
        .failed { color: red; }
        .warning { color: orange; }
    </style>
</head>
<body>
    <h1>Stremax Test Results</h1>
    <p>Date: $(date)</p>
    <h2>Summary</h2>
    <p>Total Tests: $TOTAL_TESTS</p>
    <p>Failed Tests: $FAILED_TESTS</p>
    <p>Coverage: $COVERAGE%</p>
    <h2>Detailed Results</h2>
    <pre>
$(cat $RESULTS_DIR/*.log)
    </pre>
</body>
</html>
EOF

# Print summary
echo ""
echo "Test Summary"
echo "------------"
echo "Total Tests: $TOTAL_TESTS"
echo "Failed Tests: $FAILED_TESTS"
echo "Coverage: $COVERAGE%"
echo ""
echo "Detailed results available in: $RESULTS_DIR"

# Exit with status
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi 