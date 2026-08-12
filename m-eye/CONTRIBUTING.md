# Contributing to M'Eye

Thank you for your interest in contributing to M'Eye! This document provides guidelines and instructions for contributing.

## 🎯 Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating.

## 🚀 Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/m-eye.git
cd m-eye
```

### 2. Set Up Development Environment

```bash
# Copy environment file
cp .env.example .env

# Start services
docker compose up -d
```

### 3. Build Backend

```bash
cd backend
cargo build
```

### 4. Run Tests

```bash
cargo test
```

## 📝 How to Contribute

### Reporting Bugs

1. Check existing issues first
2. Use the bug report template
3. Include:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Environment details (OS, Docker version, etc.)
   - Logs if applicable

### Suggesting Features

1. Check existing feature requests
2. Create a new issue with:
   - Problem statement
   - Proposed solution
   - Alternative solutions considered
   - Additional context

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Write/update tests**
5. **Ensure all tests pass**:
   ```bash
   cargo test
   ```
6. **Format your code**:
   ```bash
   cargo fmt
   ```
7. **Run clippy**:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
8. **Commit** with clear messages following [Conventional Commits](https://www.conventionalcommits.org/)
9. **Push** to your fork
10. **Open a Pull Request**

## 🏗️ Architecture Overview

### Backend (Rust)

```
backend/src/
├── main.rs          # Application entry point
├── api/             # HTTP route handlers
│   ├── auth.rs      # Authentication endpoints
│   ├── health.rs    # Health checks
│   ├── reputation.rs # Reputation API
│   ├── reports.rs   # Reports API
│   ├── users.rs     # User management
│   └── moderation.rs # Moderation workflows
├── core/            # Core business logic
│   ├── mod.rs       # App state, DB pool
│   ├── db.rs        # Database operations
│   ├── cache.rs     # Redis caching
│   └── security.rs  # Auth, JWT, passwords
├── models/          # Data models
│   └── mod.rs       # User, Reputation, Report, Evidence, etc.
├── smtp/            # SMTP gateway
│   └── server.rs    # SMTP server implementation
└── workers/         # Background jobs
    └── processor.rs # Email processing, scoring
```

### Key Principles

- **PostgreSQL is the source of truth**
- **Redis is for caching and queues only**
- **All decisions must be explainable**
- **UNKNOWN ≠ MALICIOUS**
- **Minimal data retention**

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_score_calculation() {
        // Test implementation
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_api_reputation_endpoint() {
    // Test API endpoint
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test api::reputation

# With output
cargo test -- --nocapture
```

## 🔐 Security Considerations

When contributing, please ensure:

1. **Input Validation**: All user inputs are validated
2. **SQL Injection Prevention**: Use parameterized queries (SQLx does this)
3. **Authentication**: Protected routes require valid JWT
4. **Authorization**: RBAC checks are performed
5. **Rate Limiting**: Implement rate limits on sensitive endpoints
6. **Logging**: Sensitive data is not logged
7. **Error Messages**: Don't leak internal information

## 📚 Documentation

- Update README.md if adding new features
- Add inline documentation for complex logic
- Update API documentation (utoipa annotations)
- Document configuration options

Example:

```rust
/// Get reputation for an email address
/// 
/// Returns the current reputation status, risk score, and confidence
/// for the specified email identity.
#[utoipa::path(
    get,
    path = "/api/v1/reputation/email/{email}",
    params(
        ("email" = String, Path, description = "Email address to check")
    ),
    responses(
        (status = 200, description = "Reputation found", body = ReputationResponse),
        (status = 404, description = "Reputation not found")
    )
)]
```

## 🎨 Code Style

We use Rust's standard formatting:

```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features
```

Follow these guidelines:

- Use descriptive variable names
- Handle errors properly (no unwrap in production code)
- Write doc comments for public APIs
- Keep functions focused and small
- Use types to encode invariants

## 🔄 Review Process

1. All PRs require at least one review
2. CI must pass (tests, clippy, fmt)
3. Address reviewer feedback promptly
4. Be respectful and constructive

## 📬 Questions?

- Open an issue for questions
- Join discussions in existing issues
- Check documentation in `docs/`

Thank you for contributing to M'Eye! 🎉
