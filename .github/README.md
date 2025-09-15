# GitHub Actions CI/CD Pipeline

This directory contains GitHub Actions workflows for the monorepo, providing comprehensive CI/CD automation for all services.

## Workflows Overview

### 1. `ci.yml` - Main Monorepo CI/CD
**Trigger**: Push/PR to main, master, develop branches
**Purpose**: Orchestrates testing for all services with change detection

**Features**:
- 🔍 **Change Detection**: Only runs tests for modified services
- 🧪 **Multi-language Support**: Rust, Node.js, Python, Go, C++
- 🚀 **Automated Deployments**: Staging and production deployments
- 🔒 **Security Scanning**: Trivy vulnerability scanning
- 📊 **Parallel Execution**: Services tested in parallel for speed

**Services Covered**:
- **Newsletter** (Rust): Cucumber BDD tests, unit tests, integration tests
- **Landing** (Next.js): Unit tests, build verification
- **UI** (React): Component tests, build verification  
- **Referral** (Python): PyTest unit tests
- **Report** (Go): Unit and integration tests
- **Stats** (C++): CMake build and CTest execution

### 2. `newsletter-ci.yml` - Newsletter Service Focused CI
**Trigger**: Changes to `newsletter/` directory
**Purpose**: Comprehensive testing pipeline for the newsletter service

**Test Coverage**:
- 🧪 **Unit Tests**: Library and binary tests
- 🥒 **Cucumber BDD Tests**: Both simple and expressions variants
- 🔧 **Code Quality**: Formatting, linting, security audit
- 🐳 **Docker Build**: Container image validation
- 📈 **Performance**: Benchmarks on main branch
- 📊 **Coverage**: Test coverage reporting with Codecov

**Key Features**:
- PostgreSQL service container for database tests
- Rust 1.89 with caching for faster builds
- Parallel test execution for cucumber variants
- Security audit with cargo-audit
- Multi-stage deployments (staging/production)

### 3. `cucumber-tests.yml` - Dedicated BDD Testing
**Trigger**: Scheduled (daily) + manual dispatch
**Purpose**: Focused cucumber test execution with detailed reporting

**Capabilities**:
- 📅 **Scheduled Runs**: Daily at 6 AM UTC
- 🎛️ **Manual Dispatch**: Run specific test types on demand
- 🌍 **Environment Selection**: Test against different environments
- 📈 **Detailed Reporting**: Test summaries and artifact uploads
- 🚨 **Failure Notifications**: Auto-create issues on scheduled failures

**Test Matrix**:
- **Simple Cucumber Tests**: Basic regex-based step definitions
- **Cucumber Expressions**: Advanced parameter type matching
- **Environment Options**: Local, staging, production

## Newsletter Service Testing Architecture

### Cucumber BDD Implementation

The newsletter service uses a comprehensive BDD testing approach with two complementary implementations:

#### 1. Basic Cucumber Tests (`cucumber_simple.rs`)
- Traditional regex-based step matching
- Comprehensive CRUD operation coverage
- 12 scenarios, 84 test steps
- In-memory test world for fast execution

#### 2. Cucumber Expressions (`cucumber_expressions.rs`)  
- Modern parameter type system
- Enhanced readability and maintainability
- 28 scenarios, 186 test steps
- Advanced features: bulk operations, domain filtering, email validation

### Parameter Types Supported
```rust
{string}  // Quoted strings: "email@example.com"
{int}     // Integers: 5, 100
{float}   // Decimals: 3.14, 99.99  
{word}    // Single words: active, inactive
```

### Test Categories
- **Create**: Email subscriptions, duplicates, bulk operations
- **Read**: Single/multiple retrievals, non-existent handling
- **Update**: Status changes, bulk updates, reactivation
- **Delete**: Single/bulk deletion, domain filtering
- **Validation**: Email format, edge cases
- **Workflows**: Complex multi-step scenarios

## Environment Configuration

### Required Environment Variables
```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/newsletter_test
RUST_LOG=info
RUST_BACKTRACE=1
```

### Service Dependencies
- **PostgreSQL 15**: Database service container
- **Protocol Buffers**: For gRPC service definitions
- **System Libraries**: libpq-dev for PostgreSQL connectivity

## Usage Examples

### Running Tests Locally
```bash
# All newsletter tests
cd newsletter && cargo test

# Specific cucumber tests  
cargo test --test cucumber_simple
cargo test --test cucumber_expressions

# With coverage
cargo tarpaulin --out html --output-dir ./coverage
```

### Manual Workflow Dispatch
1. Navigate to **Actions** tab in GitHub
2. Select **Cucumber BDD Tests** workflow
3. Click **Run workflow**
4. Choose test type and environment
5. Monitor execution and results

### Deployment Triggers
- **Staging**: Automatic on `develop` branch
- **Production**: Automatic on `main` branch  
- **Manual**: Workflow dispatch for any environment

## Monitoring and Alerts

### Test Result Artifacts
- Cucumber test results (JSON format)
- Coverage reports (XML/HTML)
- Performance benchmarks
- Security audit reports

### Failure Handling
- **Immediate**: PR status checks prevent merging
- **Scheduled**: Auto-create GitHub issues for investigation
- **Notifications**: Team alerts via configured channels

### Performance Monitoring
- Build time tracking with caching optimization
- Test execution time monitoring
- Resource usage analysis

## Contributing

### Adding New Tests
1. Create feature files in `newsletter/tests/features/`
2. Implement step definitions in test files
3. Update CI workflows if needed
4. Ensure all tests pass locally

### Modifying Workflows
1. Test changes in feature branches
2. Use workflow dispatch for validation
3. Monitor resource usage and execution time
4. Update documentation accordingly

### Best Practices
- Use change detection for efficient CI
- Implement proper caching strategies
- Write comprehensive test scenarios
- Monitor and optimize build performance
- Keep workflows maintainable and documented