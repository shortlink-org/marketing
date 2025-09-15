#!/bin/bash

# Validate Cucumber Implementation Script
# Comprehensive validation of the cucumber expressions implementation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Initialize counters
TESTS_PASSED=0
TESTS_FAILED=0
WARNINGS=0

# Function to run test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    print_info "Running: $test_name"
    
    if eval "$test_command" > /dev/null 2>&1; then
        print_success "$test_name"
        ((TESTS_PASSED++))
        return 0
    else
        print_error "$test_name"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Function to check file exists
check_file() {
    local file_path="$1"
    local description="$2"
    
    if [ -f "$file_path" ]; then
        print_success "$description exists: $file_path"
        ((TESTS_PASSED++))
        return 0
    else
        print_error "$description missing: $file_path"
        ((TESTS_FAILED++))
        return 1
    fi
}

print_header "Cucumber Expressions Implementation Validation"

# Check if we're in the right directory
if [ ! -f "newsletter/Cargo.toml" ]; then
    print_error "Must be run from workspace root directory"
    exit 1
fi

print_header "File Structure Validation"

# Check core implementation files
check_file "newsletter/tests/cucumber_expressions.rs" "Cucumber expressions test file"
check_file "newsletter/tests/cucumber_simple.rs" "Basic cucumber test file"
check_file "newsletter/tests/features/newsletter_cucumber_expressions.feature" "Advanced feature file"
check_file "newsletter/tests/features/newsletter_crud.feature" "Basic feature file"
check_file "newsletter/tests/README.md" "Test documentation"

# Check GitHub Actions workflows
check_file ".github/workflows/ci.yml" "Main CI workflow"
check_file ".github/workflows/newsletter-ci.yml" "Newsletter CI workflow"
check_file ".github/workflows/cucumber-tests.yml" "Cucumber tests workflow"
check_file ".github/README.md" "GitHub Actions documentation"

# Check Docker files
check_file "newsletter/Dockerfile" "Newsletter Dockerfile"
check_file "newsletter/.dockerignore" "Docker ignore file"

print_header "Dependency Validation"

cd newsletter

# Check Cargo.toml for required dependencies
print_info "Checking Cargo.toml dependencies..."

if grep -q 'cucumber = "0.21"' Cargo.toml; then
    print_success "Cucumber dependency found"
    ((TESTS_PASSED++))
else
    print_error "Cucumber dependency missing or wrong version"
    ((TESTS_FAILED++))
fi

if grep -q 'cucumber-expressions = "0.3"' Cargo.toml; then
    print_success "Cucumber expressions dependency found"
    ((TESTS_PASSED++))
else
    print_error "Cucumber expressions dependency missing or wrong version"
    ((TESTS_FAILED++))
fi

if grep -q 'rust-version = "1.89"' Cargo.toml; then
    print_success "Rust version set to 1.89"
    ((TESTS_PASSED++))
else
    print_warning "Rust version not set to 1.89"
    ((WARNINGS++))
fi

print_header "Code Quality Validation"

# Check compilation
run_test "Project compilation" "cargo check --all-targets"

# Check formatting
if cargo fmt --all -- --check > /dev/null 2>&1; then
    print_success "Code formatting is correct"
    ((TESTS_PASSED++))
else
    print_warning "Code formatting issues found (can be auto-fixed)"
    ((WARNINGS++))
fi

# Check clippy
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    print_success "No clippy warnings"
    ((TESTS_PASSED++))
else
    print_warning "Clippy warnings found"
    ((WARNINGS++))
fi

print_header "Test Execution Validation"

# Run unit tests
run_test "Unit tests" "cargo test --lib --bins"

# Run cucumber tests
run_test "Basic cucumber tests" "cargo test --test cucumber_simple"
run_test "Cucumber expressions tests" "cargo test --test cucumber_expressions"

print_header "Feature Coverage Analysis"

# Count scenarios in feature files
BASIC_SCENARIOS=$(grep -c "Scenario:" tests/features/newsletter_crud.feature || echo "0")
ADVANCED_SCENARIOS=$(grep -c "Scenario:" tests/features/newsletter_cucumber_expressions.feature || echo "0")
TOTAL_SCENARIOS=$((BASIC_SCENARIOS + ADVANCED_SCENARIOS))

print_info "Basic feature scenarios: $BASIC_SCENARIOS"
print_info "Advanced feature scenarios: $ADVANCED_SCENARIOS"
print_info "Total scenarios: $TOTAL_SCENARIOS"

if [ "$TOTAL_SCENARIOS" -ge 25 ]; then
    print_success "Comprehensive scenario coverage ($TOTAL_SCENARIOS scenarios)"
    ((TESTS_PASSED++))
else
    print_warning "Limited scenario coverage ($TOTAL_SCENARIOS scenarios)"
    ((WARNINGS++))
fi

print_header "Parameter Type Validation"

# Check for parameter type usage in cucumber expressions
PARAM_TYPES=("string" "int" "float" "word")
for param_type in "${PARAM_TYPES[@]}"; do
    if grep -q "{$param_type}" tests/cucumber_expressions.rs; then
        print_success "Parameter type {$param_type} implemented"
        ((TESTS_PASSED++))
    else
        print_warning "Parameter type {$param_type} not found"
        ((WARNINGS++))
    fi
done

print_header "Documentation Validation"

# Check for comprehensive documentation
if [ -f "tests/README.md" ]; then
    DOC_LINES=$(wc -l < tests/README.md)
    if [ "$DOC_LINES" -gt 100 ]; then
        print_success "Comprehensive test documentation ($DOC_LINES lines)"
        ((TESTS_PASSED++))
    else
        print_warning "Limited test documentation ($DOC_LINES lines)"
        ((WARNINGS++))
    fi
fi

cd ..

print_header "GitHub Actions Validation"

# Check workflow file structure
for workflow in ci.yml newsletter-ci.yml cucumber-tests.yml; do
    if [ -f ".github/workflows/$workflow" ]; then
        # Check for key sections
        if grep -q "cucumber" ".github/workflows/$workflow"; then
            print_success "Workflow $workflow includes cucumber tests"
            ((TESTS_PASSED++))
        else
            print_warning "Workflow $workflow may not include cucumber tests"
            ((WARNINGS++))
        fi
    fi
done

print_header "Docker Validation"

cd newsletter

# Test Docker build (if Docker is available)
if command -v docker &> /dev/null; then
    print_info "Testing Docker build..."
    if docker build -t newsletter:validation-test . > /dev/null 2>&1; then
        print_success "Docker build successful"
        ((TESTS_PASSED++))
        # Cleanup
        docker rmi newsletter:validation-test > /dev/null 2>&1 || true
    else
        print_warning "Docker build failed (may need dependencies)"
        ((WARNINGS++))
    fi
else
    print_warning "Docker not available for build testing"
    ((WARNINGS++))
fi

cd ..

print_header "Final Validation Summary"

echo -e "\n📊 Results:"
echo -e "  ${GREEN}✅ Tests Passed: $TESTS_PASSED${NC}"
echo -e "  ${RED}❌ Tests Failed: $TESTS_FAILED${NC}"
echo -e "  ${YELLOW}⚠️  Warnings: $WARNINGS${NC}"

# Calculate success rate
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
if [ "$TOTAL_TESTS" -gt 0 ]; then
    SUCCESS_RATE=$((TESTS_PASSED * 100 / TOTAL_TESTS))
    echo -e "  📈 Success Rate: $SUCCESS_RATE%"
fi

echo -e "\n🎯 Implementation Status:"
if [ "$TESTS_FAILED" -eq 0 ]; then
    if [ "$WARNINGS" -eq 0 ]; then
        echo -e "  ${GREEN}🎉 Perfect! Ready for merge${NC}"
        exit 0
    else
        echo -e "  ${YELLOW}✨ Good! Minor issues to address${NC}"
        echo -e "  ${BLUE}💡 Consider fixing warnings before merge${NC}"
        exit 0
    fi
else
    echo -e "  ${RED}🚨 Issues found! Fix before merge${NC}"
    echo -e "  ${BLUE}💡 Address failed tests and try again${NC}"
    exit 1
fi