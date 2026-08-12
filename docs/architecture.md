# M'Eye Architecture

## Overview

M'Eye is designed with simplicity and modularity in mind. The architecture follows a microservices-inspired approach while maintaining ease of deployment.

## Core Components

```
                    ┌──────────────┐
                    │   M'Eye Web  │
                    │   + API      │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              │                         │
       ┌──────▼──────┐           ┌──────▼──────┐
       │ PostgreSQL  │           │    Redis    │
       │             │           │             │
       └─────────────┘           └──────┬──────┘
                                        │
                                 ┌──────▼──────┐
                                 │    Worker   │
                                 │ SMTP/Jobs   │
                                 └─────────────┘
```

## Services

### 1. Backend (FastAPI)

**Port**: 8000 (API), 3000 (Web UI)

The backend handles:
- REST API endpoints
- Authentication & Authorization
- Reputation queries
- Report management
- User management
- Web UI serving (Phase 2+)

**Technology Stack**:
- FastAPI for REST API
- SQLAlchemy for ORM
- Pydantic for validation
- JWT for authentication

### 2. PostgreSQL Database

**Port**: 5432

Stores all persistent data:
- Users and roles
- Reputation records
- Reports and evidence
- Suggestions
- Audit logs

**Key Tables**:
- `users` - User accounts with RBAC
- `reputations` - Identity reputation scores
- `reputation_history` - Change tracking
- `reports` - Community reports
- `evidence` - Supporting evidence
- `suggestions` - Status change proposals

### 3. Redis

**Port**: 6379

Used for:
- Caching reputation lookups
- Rate limiting
- Session storage
- Task queues (future)

### 4. Worker

**Port**: 2525 (SMTP)

Handles:
- SMTP gateway operations
- Email parsing
- Authentication checks (SPF/DKIM/DMARC)
- Async job processing
- Threat intelligence updates (Phase 3)

## Data Flow

### Email Analysis Flow

```
Email Received
     ↓
SMTP Gateway (Worker)
     ↓
Parse Headers & Content
     ↓
Check SPF/DKIM/DMARC
     ↓
Query Reputation (PostgreSQL/Redis)
     ↓
Calculate Risk Score
     ↓
Decision: ALLOW / WATCH / BLOCK
     ↓
Log Results
```

### Reputation Query Flow

```
API Request
     ↓
Authenticate User
     ↓
Check Redis Cache
     ↓
[Cache Miss] Query PostgreSQL
     ↓
Return Response
     ↓
Update Cache
```

### Report Submission Flow

```
User Submits Report
     ↓
Validate Input
     ↓
Store in PostgreSQL
     ↓
Attach Evidence
     ↓
Update Reputation Count
     ↓
Trigger Review (if needed)
```

## Security Architecture

### Defense in Depth

1. **Network Level**
   - Container isolation
   - Port exposure control
   - Firewall rules (user-configured)

2. **Application Level**
   - Input validation (Pydantic)
   - SQL injection prevention (ORM)
   - XSS protection
   - CSRF tokens

3. **Authentication**
   - JWT-based auth
   - Password hashing (bcrypt)
   - Role-based access control
   - Rate limiting

4. **Data Protection**
   - Secrets in environment variables
   - No sensitive data in logs
   - Minimal data retention

## Extensibility

### Adding New Providers

Threat intelligence providers can be added by implementing the `ThreatIntelProvider` interface:

```python
class ThreatIntelProvider:
    def query(self, identity: str) -> dict
    def get_confidence(self) -> float
    def get_name(self) -> str
```

### Adding New Identity Types

Extend the `IdentityType` enum and add validation logic in the reputation service.

### Adding New Report Categories

Update the `ReportCategory` enum and modify the risk calculation weights.

## Scaling Considerations

### Phase 1 (MVP)
- Single instance of each service
- Suitable for small to medium deployments

### Phase 2-3
- Multiple worker instances
- Redis cluster for caching
- Read replicas for PostgreSQL

### Phase 4 (Production)
- Kubernetes orchestration
- Horizontal pod autoscaling
- Distributed task queues
- OpenSearch for analytics

## Monitoring

### Health Checks

Each service exposes health endpoints:
- `/api/v1/health` - Backend health
- PostgreSQL `pg_isready`
- Redis `PING`

### Metrics to Track

- API response times
- Database query performance
- Cache hit rates
- Email processing volume
- Report submission rates

## Disaster Recovery

### Backup Strategy

1. **PostgreSQL**: Daily backups with point-in-time recovery
2. **Redis**: Optional persistence (AOF)
3. **Configuration**: Version-controlled docker-compose and .env

### Recovery Steps

1. Restore PostgreSQL from backup
2. Restart services
3. Verify data integrity
4. Update DNS if needed

---

For implementation details, see the codebase and inline documentation.
