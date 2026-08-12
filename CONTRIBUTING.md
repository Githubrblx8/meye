# Contributing to M'Eye

Thank you for considering contributing to M'Eye! This document provides guidelines for contributing to the project.

## 🎯 Project Goals

M'Eye aims to be:
- **Open Source**: Fully transparent and community-driven
- **Self-hostable**: Easy to install with minimal dependencies
- **Secure**: Following security best practices
- **Modular**: Easy to extend and customize
- **Explainable**: All decisions should be traceable and explainable

## 📋 Code of Conduct

Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## 🚀 Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/m-eye.git
cd m-eye
```

### 2. Set up Development Environment

```bash
# Copy environment file
cp .env.example .env

# Start services
docker compose up -d

# Run tests
docker compose exec backend pytest
```

### 3. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

## 📝 Development Guidelines

### Code Style

- Use type hints in Python code
- Follow PEP 8 style guide
- Write docstrings for public functions and classes
- Keep functions small and focused

### Testing

- Write unit tests for new features
- Ensure all tests pass before submitting PR
- Aim for good test coverage

### Security

- Never commit secrets or credentials
- Validate all user inputs
- Follow principle of least privilege
- Document security implications of changes

## 🔄 Pull Request Process

1. **Create a PR** with a clear title and description
2. **Link issues** that the PR addresses
3. **Ensure tests pass**
4. **Update documentation** if needed
5. **Request review** from maintainers

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No security issues introduced
```

## 📁 Project Structure

```
m-eye/
├── backend/          # FastAPI backend
│   ├── app/
│   │   ├── api/     # API endpoints
│   │   ├── models/  # Database models
│   │   ├── services/# Business logic
│   │   └── core/    # Configuration, security
│   └── requirements.txt
├── worker/           # SMTP gateway & async jobs
├── frontend/         # Web UI (Phase 2+)
├── migrations/       # Database migrations
├── docs/            # Documentation
└── docker-compose.yml
```

## 🧪 Running Tests

```bash
# Run all tests
docker compose exec backend pytest

# Run with coverage
docker compose exec backend pytest --cov=app

# Run specific test file
docker compose exec backend pytest tests/test_reputation.py
```

## 📖 Documentation

- Update README.md for significant changes
- Add inline comments for complex logic
- Update API documentation if endpoints change

## 🔐 Security Reporting

If you find a security vulnerability, please report it privately by emailing security@m-eye.local (when configured) or opening a private issue.

See [SECURITY.md](SECURITY.md) for details.

## 💡 Ideas for Contribution

### Phase 1 (Current)
- [ ] Improve email parsing
- [ ] Add more SPF/DKIM/DMARC checks
- [ ] Enhance reputation algorithm
- [ ] Add unit tests

### Phase 2
- [ ] Web UI components
- [ ] Evidence upload system
- [ ] Moderation dashboard
- [ ] RBAC improvements

### Phase 3
- [ ] OpenSearch integration
- [ ] Threat intelligence providers
- [ ] Advanced analytics
- [ ] Researcher dashboard

### Phase 4
- [ ] Production hardening
- [ ] Monitoring & alerting
- [ ] Distributed workers
- [ ] Federation support

## 🤝 Questions?

Feel free to open an issue for questions or discussions.

---

Thank you for contributing to M'Eye! 🎉
