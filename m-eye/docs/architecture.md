# M'Eye Architecture

## Overview

M'Eye is built with a modular architecture designed for security, performance, and ease of deployment.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Client Layer                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Web UI    │  │  REST API   │  │   SMTP Gateway      │  │
│  │  (React)    │  │  Clients    │  │   (Port 2525)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                          │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Rust Backend (Axum)                     │   │
│  │                                                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌──────────────┐   │   │
│  │  │   Auth     │  │Reputation  │  │   Reports    │   │   │
│  │  │   Module   │  │   Engine   │  │   Module     │   │   │
│  │  └────────────┘  └────────────┘  └──────────────┘   │   │
│  │                                                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌──────────────┐   │   │
│  │  │   Risk     │  │Moderation  │  │   Users      │   │   │
│  │  │   Engine   │  │   System   │  │   Module     │   │   │
│  │  └────────────┘  └────────────┘  └──────────────┘   │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Data Layer                                │
│                                                              │
│  ┌─────────────────┐              ┌─────────────────────┐   │
│  │   PostgreSQL    │              │       Redis         │   │
│  │  (Source of     │              │  (Cache / Queue)    │   │
│  │   Truth)        │              │                     │   │
│  │                 │              │  - Sessions         │   │
│  │  - Users        │              │  - Rate Limits      │   │
│  │  - Reputations  │              │  - Job Queues       │   │
│  │  - Reports      │              │  - Reputation Cache │   │
│  │  - Evidence     │              │                     │   │
│  │  - Suggestions  │              │                     │   │
│  │  - Audit Logs   │              │                     │   │
│  └─────────────────┘              └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Components

### Backend (Rust + Axum)

**Purpose**: Core business logic, API, authentication, reputation engine

**Key Features**:
- High-performance async runtime (Tokio)
- Type-safe database queries (SQLx)
- JWT-based authentication
- RBAC implementation
- Explainable risk scoring
- OpenAPI documentation (utoipa)

**Modules**:
- `api/`: HTTP route handlers
- `core/`: Business logic, DB pool, cache
- `models/`: Data structures and database mappings
- `smtp/`: SMTP server implementation
- `workers/`: Background job processors

### Database (PostgreSQL 15)

**Purpose**: Persistent storage, source of truth

**Key Tables**:
- `users`: User accounts with roles
- `reputations`: Identity reputation data
- `reports`: Community reports
- `evidence`: Supporting evidence for reports
- `suggestions`: Classification suggestions
- `reputation_history`: Audit trail
- `smtp_logs`: Email processing logs

**Design Principles**:
- All tables use UUID primary keys
- Timestamps in UTC (TIMESTAMPTZ)
- Proper indexing for query performance
- Foreign key constraints for data integrity
- Enum types for fixed values

### Cache/Queue (Redis 7)

**Purpose**: Performance optimization, async processing

**Use Cases**:
- Session caching
- Rate limiting counters
- Reputation score cache
- Background job queues
- Temporary analysis results

**Important**: Redis is NEVER the source of truth. All critical data is persisted to PostgreSQL.

### SMTP Gateway

**Purpose**: Email analysis pipeline

**Flow**:
```
Email Received
     ↓
Parse Headers & Body
     ↓
SPF Check
     ↓
DKIM Verification
     ↓
DMARC Validation
     ↓
Threat Intelligence Lookup
     ↓
Reputation Check
     ↓
Risk Score Calculation
     ↓
Decision: ALLOW / WATCH / BLOCK
```

**Features**:
- Inbound and outbound analysis
- Header parsing and validation
- Authentication protocol verification
- Integration with reputation engine
- Configurable policies

## Data Flow

### Reputation Check Flow

```
API Request: GET /api/v1/reputation/email/user@example.com
                    ↓
            Authentication Check
                    ↓
             Rate Limiting (Redis)
                    ↓
          Cache Lookup (Redis)
                    ↓
            [Cache Miss]
                    ↓
        Database Query (PostgreSQL)
                    ↓
           Build Response
                    ↓
        Update Cache (Redis)
                    ↓
            Return JSON Response
```

### Report Submission Flow

```
POST /api/v1/reports
        ↓
  Authenticate User
        ↓
  Validate Input
        ↓
  Check Rate Limits
        ↓
  Create Report (Pending)
        ↓
  Store Evidence
        ↓
  Queue for Review
        ↓
  Notify Moderators
        ↓
  Return Success
```

### Email Processing Flow

```
SMTP Connection (Port 2525)
        ↓
   Parse Email
        ↓
 Extract Sender/Recipient
        ↓
   Check SPF/DKIM/DMARC
        ↓
  Lookup Sender Reputation
        ↓
  Calculate Risk Score
        ↓
   Apply Policy Rules
        ↓
  Make Decision
        ↓
  Log Result
        ↓
  Forward / Quarantine / Reject
```

## Security Architecture

### Authentication

- JWT tokens with configurable expiration
- Argon2id password hashing
- Rate limiting on login attempts
- Secure cookie handling (for web UI)

### Authorization

Role-Based Access Control (RBAC):

| Role | Permissions |
|------|-------------|
| Member | View reputations, submit reports, manage personal blocklists |
| Trusted Reporter | Enhanced report weight, priority review |
| Security Researcher | Publish analyses, advanced evidence submission |
| Moderator | Review reports, approve/reject suggestions, modify classifications |
| Administrator | Full system access, user management, configuration |

### Input Validation

- Strict type validation with Serde
- Email format validation
- URL sanitization
- SQL injection prevention (parameterized queries)
- XSS prevention (output encoding)

### Audit Trail

All security-relevant events are logged:
- Authentication attempts
- Authorization failures
- Reputation changes
- Moderation decisions
- Configuration modifications

## Deployment Architecture

### Development

```
┌─────────────────┐
│   Developer     │
│   Machine       │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────┐
│       Docker Compose            │
│                                 │
│  ┌─────────┐  ┌──────────────┐  │
│  │ Postgres│  │    Redis     │  │
│  └─────────┘  └──────────────┘  │
│                                 │
│  ┌──────────────────────────┐   │
│  │   Backend (Rust)         │   │
│  │   - API (8000)           │   │
│  │   - SMTP (2525)          │   │
│  └──────────────────────────┘   │
└─────────────────────────────────┘
```

### Production (Recommended)

```
                    ┌─────────────────┐
                    │   Load Balancer │
                    │   (nginx/traefik)│
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
     ┌────────────┐ ┌────────────┐ ┌────────────┐
     │  Backend   │ │  Backend   │ │  Backend   │
     │  Instance  │ │  Instance  │ │  Instance  │
     └─────┬──────┘ └─────┬──────┘ └─────┬──────┘
           │              │              │
           └──────────────┼──────────────┘
                          │
              ┌───────────┴───────────┐
              │                       │
              ▼                       ▼
     ┌─────────────────┐     ┌───────────────┐
     │   PostgreSQL    │     │     Redis     │
     │   (Primary)     │     │   (Cluster)   │
     └─────────────────┘     └───────────────┘
```

## Scalability Considerations

### Current Design (MVP)

- Single backend instance
- Single PostgreSQL instance
- Single Redis instance
- Suitable for: Development, small organizations, testing

### Future Scaling (Phase 4+)

- Multiple backend instances behind load balancer
- PostgreSQL read replicas
- Redis cluster
- Distributed job queues
- Horizontal scaling based on load

## Monitoring & Observability

### Metrics to Track

- API response times
- Database query performance
- Cache hit rates
- Email processing volume
- Reputation lookup frequency
- Error rates by endpoint

### Logging Strategy

- Structured logging (JSON format)
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- Correlation IDs for request tracing
- Sensitive data redaction

## Disaster Recovery

### Backup Strategy

1. **PostgreSQL**: Daily automated backups with WAL archiving
2. **Redis**: RDB snapshots for cache persistence (optional)
3. **Configuration**: Version-controlled environment files

### Recovery Procedures

1. Restore PostgreSQL from backup
2. Apply WAL logs to desired point
3. Restart application containers
4. Verify health checks
5. Monitor for anomalies

## Technology Choices Rationale

| Technology | Why? |
|------------|------|
| Rust | Memory safety, performance, excellent async support |
| Axum | Modern, ergonomic, great Tokio integration |
| PostgreSQL | Reliability, ACID compliance, advanced features |
| Redis | Speed, simplicity, perfect for caching/queues |
| Docker Compose | Simple deployment, reproducible environments |
| AGPL-3.0 | Ensures modifications remain open source |

---

This architecture is designed to evolve. Start simple with the MVP, then scale components as needed based on actual usage patterns.
