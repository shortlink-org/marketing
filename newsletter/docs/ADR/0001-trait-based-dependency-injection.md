# ADR-0001: Trait-Based Dependency Injection for Newsletter Service

## Status
**ACCEPTED** - Implemented on 2025-09-14

## Context

The newsletter service was initially implemented with direct database access patterns where business logic, data access, and API layers were tightly coupled. This approach presented several challenges:

1. **Testing Difficulties**: Unit testing required setting up actual database connections, making tests slow and brittle
2. **Tight Coupling**: Business logic was directly dependent on specific database implementations
3. **Poor Maintainability**: Changes to database schema or switching database providers required modifications across multiple layers
4. **Limited Extensibility**: Adding new features or alternative implementations was complex due to hardcoded dependencies
5. **Violation of SOLID Principles**: The code violated the Dependency Inversion Principle by depending on concrete implementations rather than abstractions

### Technical Context

- **Language**: Rust 1.89
- **Framework**: Tonic (gRPC), Diesel (ORM), Tokio (async runtime)
- **Database**: PostgreSQL with async connection pooling
- **Architecture**: Clean Architecture principles desired
- **Deployment**: Containerized microservice

### Current Pain Points

```rust
// Before: Direct database access in gRPC handlers
async fn subscribe(&self, req: Request<SubscribeRequest>) -> Result<Response<()>, Status> {
    let email = req.into_inner().email;
    repo::add(&self.pool, &email)  // Direct dependency on concrete implementation
        .await
        .map_err(|e| Status::internal(format!("db error: {e}")))?;
    Ok(Response::new(()))
}
```

## Decision

We will implement **Trait-Based Dependency Injection** using Rust's trait system to create a layered architecture with clear separation of concerns.

### Architecture Design

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   gRPC Layer    │───▶│  Service Layer  │───▶│ Repository Layer│
│ (Presentation)  │    │ (Business Logic)│    │ (Data Access)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
   Depends on              Depends on              Implements
        │                       │                       │
        ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│NewsletterService│    │NewsletterService│    │NewsletterRepository│
│   (gRPC Impl)   │    │    (Trait)      │    │    (Trait)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Implementation Strategy

1. **Repository Layer**
   - Define `NewsletterRepository` trait for data access abstraction
   - Implement `PostgresNewsletterRepository` as concrete implementation
   - Support for future implementations (Redis, MongoDB, etc.)

2. **Service Layer**
   - Define `NewsletterService` trait for business logic abstraction
   - Implement `DefaultNewsletterService` with injected repository dependency
   - Handle validation, business rules, and error handling

3. **Presentation Layer**
   - Update gRPC service to depend on `NewsletterService` trait
   - Remove direct database dependencies
   - Focus solely on protocol translation

4. **Dependency Injection**
   - Use `Arc<dyn Trait>` for shared ownership across async boundaries
   - Wire dependencies in `main.rs` using constructor injection
   - Leverage Rust's type system for compile-time dependency validation

5. **Built-in Demonstration**
   - Include `demo_functionality` method in service trait
   - Expose demo via gRPC endpoint for live testing
   - Showcase dependency injection in action

## Consequences

### Positive

1. **Improved Testability**
   ```rust
   // Easy mocking for unit tests
   struct MockNewsletterRepository;
   impl NewsletterRepository for MockNewsletterRepository { /* ... */ }
   
   let mock_repo = Arc::new(MockNewsletterRepository);
   let service = DefaultNewsletterService::new(mock_repo);
   ```

2. **Enhanced Maintainability**
   - Clear separation of concerns
   - Single Responsibility Principle adherence
   - Easier code navigation and understanding

3. **Increased Flexibility**
   ```rust
   // Easy to swap implementations
   let repository: Arc<dyn NewsletterRepository> = if use_redis {
       Arc::new(RedisNewsletterRepository::new(redis_client))
   } else {
       Arc::new(PostgresNewsletterRepository::new(pg_pool))
   };
   ```

4. **Better Error Handling**
   - Centralized business logic validation
   - Consistent error propagation patterns
   - Domain-specific error types

5. **Type Safety**
   - Compile-time dependency validation
   - Trait bounds ensure interface compliance
   - Prevents runtime dependency injection errors

### Negative

1. **Increased Complexity**
   - More files and abstractions to maintain
   - Steeper learning curve for new developers
   - Additional trait bounds and generic constraints

2. **Runtime Performance**
   - Dynamic dispatch overhead with `dyn Trait`
   - Additional memory allocations for `Arc<T>`
   - Negligible impact for I/O-bound operations

3. **Compilation Time**
   - Increased compile times due to generic instantiation
   - More complex type checking
   - Additional trait resolution

### Neutral

1. **Code Volume**
   - More lines of code due to trait definitions
   - Offset by reduced duplication and better organization

2. **Learning Curve**
   - Requires understanding of Rust trait system
   - Benefits outweigh initial complexity

## Implementation Details

### Repository Trait Definition

```rust
#[async_trait]
pub trait NewsletterRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Newsletter>>;
    async fn add(&self, email: &str) -> Result<()>;
    async fn delete(&self, email: &str) -> Result<()>;
    async fn get_by_email(&self, email: &str) -> Result<Option<Newsletter>>;
}
```

### Service Trait Definition

```rust
#[async_trait]
pub trait NewsletterService: Send + Sync {
    async fn list_newsletters(&self) -> Result<Vec<Newsletter>>;
    async fn subscribe(&self, email: &str) -> Result<()>;
    async fn unsubscribe(&self, email: &str) -> Result<()>;
    async fn get_subscription_status(&self, email: &str) -> Result<bool>;
    async fn update_subscription_status(&self, emails: Vec<String>, active: bool) -> Result<()>;
    async fn delete_subscriptions(&self, emails: Vec<String>) -> Result<()>;
    async fn demo_functionality(&self, test_email: &str) -> Result<String>;
}
```

### Dependency Injection Wiring

```rust
// main.rs
let repository = Arc::new(PostgresNewsletterRepository::new(pool.clone()));
let newsletter_service = Arc::new(DefaultNewsletterService::new(repository));
let grpc_service = MyNewsletterService::new(newsletter_service);
```

## Alternatives Considered

### 1. Service Locator Pattern
- **Rejected**: Hidden dependencies, runtime errors, harder to test
- **Reason**: Violates explicit dependency principle

### 2. Concrete Dependency Injection Container
- **Considered**: Using crates like `shaku` or `di`
- **Rejected**: Added complexity without significant benefits for current scope
- **Future consideration**: May revisit for larger applications

### 3. Compile-time Dependency Injection
- **Considered**: Zero-cost abstractions with generics only
- **Rejected**: Would require generic propagation throughout the call stack
- **Trade-off**: Chose runtime flexibility over compile-time optimization

### 4. Module-based Organization
- **Considered**: Organizing by feature modules rather than layers
- **Partial adoption**: Combined with trait-based injection for best of both

## Compliance

This decision aligns with:

- **SOLID Principles**: Especially Dependency Inversion and Single Responsibility
- **Clean Architecture**: Clear dependency direction and layer separation
- **Rust Best Practices**: Leveraging type system for safety and correctness
- **Microservice Patterns**: Testable, maintainable service design

## Metrics and Monitoring

Success will be measured by:

1. **Test Coverage**: Ability to achieve >90% unit test coverage
2. **Build Times**: Acceptable increase in compilation time (<20%)
3. **Runtime Performance**: No significant performance degradation
4. **Developer Productivity**: Reduced time for feature implementation
5. **Bug Reduction**: Fewer production issues related to data access

## Migration Path

1. **Phase 1**: Implement traits and new implementations alongside existing code
2. **Phase 2**: Update gRPC layer to use new service layer
3. **Phase 3**: Migrate main.rs to use dependency injection
4. **Phase 4**: Mark old direct database functions as deprecated
5. **Phase 5**: Remove deprecated code after validation period

## Related ADRs

- ADR-0002: Database Connection Pool Management (Future)
- ADR-0003: Error Handling Strategy (Future)
- ADR-0004: Testing Strategy for Microservices (Future)

## References

- [Clean Architecture by Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Rust Async Trait Documentation](https://docs.rs/async-trait/)
- [Dependency Injection in Rust](https://blog.logrocket.com/dependency-injection-rust/)
- [SOLID Principles](https://en.wikipedia.org/wiki/SOLID)

---

**Author**: AI Assistant  
**Reviewers**: [To be filled by team]  
**Date**: 2025-09-14  
**Last Updated**: 2025-09-14