# M'Eye

**Open Source Email Security and Reputation Platform**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue.svg)](https://www.docker.com/)

M'Eye is an open-source email security and reputation platform that allows individuals, organizations, administrators, and cybersecurity researchers to monitor incoming and outgoing emails through an SMTP gateway.

## 🎯 Features

- **Email Reputation System**: Determine if an email, domain, or IP is SAFE, UNKNOWN, WATCH, BLOCKED, or COMPROMISED
- **SMTP Gateway**: Analyze inbound and outbound emails
- **Authentication Checks**: SPF, DKIM, DMARC validation
- **Risk Engine**: Explainable risk scoring
- **Community Reporting**: Users can report suspicious emails with evidence
- **RBAC**: Role-based access control (Member, Trusted Reporter, Researcher, Moderator, Admin)
- **Audit Trail**: Complete history of reputation changes
- **REST API**: Fully documented OpenAPI interface

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose installed
- No need to install Rust, PostgreSQL, Redis, or any other dependencies!

### Installation

```bash
# Clone the repository
git clone https://github.com/your-user/m-eye.git
cd m-eye

# Run the installation script
./install.sh
```

Or manually:

```bash
cp .env.example .env
docker compose up -d
```

### Access

- **Web Interface**: http://localhost:3000
- **API**: http://localhost:8000
- **API Documentation**: http://localhost:8000/docs
- **SMTP Gateway**: localhost:2525

### Default Admin Account

- **Email**: `admin@m-eye.local`
- **Password**: `ChangeMe123!`

⚠️ **Important**: Change the default password immediately!

## 🏗️ Architecture

```
                    ┌──────────────┐
                    │   M'Eye      │
                    │   Backend    │
                    │   (Rust)     │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              │                         │
       ┌──────▼──────┐           ┌──────▼──────┐
       │ PostgreSQL  │           │    Redis    │
       │  (Source    │           │  (Cache/    │
       │   of Truth) │           │   Queue)    │
       └─────────────┘           └─────────────┘
```

### Components

| Component | Technology | Purpose |
|-----------|------------|---------|
| Backend API | Rust + Axum | REST API, business logic |
| SMTP Gateway | Rust | Email parsing, SPF/DKIM/DMARC |
| Database | PostgreSQL | Source of truth for all data |
| Cache/Queue | Redis | Caching, rate limiting, async jobs |
| Frontend | React + TypeScript | Web dashboard (Phase 2) |

## 📊 Reputation Status

| Status | Score | Description |
|--------|-------|-------------|
| 🟢 SAFE | 0-20 | Trusted identity |
| ⚪ UNKNOWN | 21-40 | Insufficient information |
| 🟠 WATCH | 41-60 | Suspicious, monitor closely |
| 🔴 BLOCKED | 61-80 | Confirmed malicious |
| 🟣 COMPROMISED | 81-100 | Compromised or associated with breach |

## 🔌 API Examples

### Check Email Reputation

```bash
curl http://localhost:8000/api/v1/reputation/email/phishing@example.com
```

### Submit a Report

```bash
curl -X POST http://localhost:8000/api/v1/reports \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "target": "phishing@example.com",
    "target_type": "email",
    "category": "phishing",
    "description": "Suspicious phishing attempt"
  }'
```

### Get API Documentation

Visit http://localhost:8000/docs for interactive Swagger UI.

## 🛡️ Security Principles

1. **UNKNOWN ≠ MALICIOUS**: Unknown identities are not automatically considered malicious
2. **Explainable Decisions**: All classifications include reasoning
3. **Audit Trail**: Every decision is logged and traceable
4. **Evidence-Based**: Reports require evidence, not just opinions
5. **Minimal Data**: Personal data is minimized
6. **Rate Limiting**: Protection against abuse
7. **RBAC**: Principle of least privilege

## 📖 Documentation

- [Architecture](docs/architecture.md)
- [Installation Guide](docs/installation.md)
- [API Reference](http://localhost:8000/docs)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## 🗺️ Roadmap

### Phase 1 (Current) ✅
- [x] PostgreSQL schema
- [x] Redis integration
- [x] SMTP Gateway
- [x] Email parser
- [x] SPF/DKIM/DMARC analysis
- [x] Reputation Engine
- [x] Risk Score (explainable)
- [x] REST API
- [x] Authentication (JWT)
- [x] Basic dashboard

### Phase 2
- [ ] Community reports
- [ ] Evidence submission
- [ ] Suggestions system
- [ ] Moderation workflow
- [ ] Full RBAC implementation
- [ ] React dashboard

### Phase 3
- [ ] OpenSearch integration
- [ ] Threat Intelligence providers
- [ ] Researcher dashboard
- [ ] Advanced analytics
- [ ] Community reputation

### Phase 4
- [ ] Production hardening
- [ ] Monitoring & alerting
- [ ] Distributed workers
- [ ] Advanced threat detection
- [ ] Federation/community reputation

## 🧪 Development

### Run Tests

```bash
cd backend
cargo test
```

### Build from Source

```bash
cd backend
cargo build --release
```

### View Logs

```bash
docker compose logs -f
```

### Stop Services

```bash
docker compose down
```

To remove volumes:

```bash
docker compose down -v
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

M'Eye is inspired by community-driven security initiatives and the belief that email security should be transparent, collaborative, and accessible to everyone.

---

Built with 🦀 Rust and ❤️ by the M'Eye Team
