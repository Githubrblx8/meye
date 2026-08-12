# M'Eye Installation Guide

This guide covers installation and configuration of M'Eye for development and production environments.

## Prerequisites

### Required

- **Docker** (version 20.10 or higher)
- **Docker Compose** (version 2.0 or higher)

### Optional (for development)

- Rust toolchain (1.75+)
- PostgreSQL client tools (`psql`)
- Redis CLI (`redis-cli`)
- curl or httpie for API testing

## Quick Start (Recommended)

The easiest way to install M'Eye is using the provided installation script:

```bash
# Clone the repository
git clone https://github.com/your-user/m-eye.git
cd m-eye

# Run the installation script
./install.sh
```

The script will:
1. Verify Docker installation
2. Create `.env` file with secure secrets
3. Start all containers
4. Wait for services to be healthy
5. Display access information

### Manual Installation

If you prefer to configure manually:

```bash
# Clone the repository
git clone https://github.com/your-user/m-eye.git
cd m-eye

# Copy environment file
cp .env.example .env

# Edit .env with your settings (optional, defaults work for local dev)
nano .env

# Start services
docker compose up -d

# View logs
docker compose logs -f
```

## Configuration

### Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# PostgreSQL Configuration
POSTGRES_USER=m_eye                    # Database user
POSTGRES_PASSWORD=your_secure_password # CHANGE THIS!
POSTGRES_DB=m_eye                      # Database name

# JWT Secret - Generate a strong random value
# openssl rand -base64 32
JWT_SECRET=your_jwt_secret_key         # CHANGE THIS!

# API Configuration
API_HOST=0.0.0.0
API_PORT=8000

# SMTP Gateway
SMTP_PORT=2525

# Logging
RUST_LOG=info                          # debug, info, warn, error
```

### Generating Secure Secrets

```bash
# JWT Secret (min 32 bytes)
openssl rand -base64 32

# Database Password (min 16 bytes)
openssl rand -base64 32 | tr -d '/='
```

### Production Configuration

For production deployments, additionally configure:

```bash
# Use external database (recommended for production)
DATABASE_URL=postgresql://user:pass@db.example.com:5432/m_eye

# External Redis
REDIS_URL=redis://redis.example.com:6379

# Secure logging
RUST_LOG=warn

# Additional security settings
API_HOST=127.0.0.1  # Bind to localhost, use reverse proxy
```

## Accessing M'Eye

After installation, access the services:

| Service | URL | Description |
|---------|-----|-------------|
| Web UI | http://localhost:3000 | Dashboard (Phase 2) |
| API | http://localhost:8000 | REST API |
| API Docs | http://localhost:8000/docs | Swagger UI |
| SMTP Gateway | localhost:2525 | Email processing |

### Default Admin Account

```
Email:    admin@m-eye.local
Password: ChangeMe123!
```

⚠️ **IMPORTANT**: Change the default password immediately after first login!

## Managing Services

### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f backend
docker compose logs -f postgres
docker compose logs -f redis
```

### Restart Services

```bash
# Restart all
docker compose restart

# Restart specific service
docker compose restart backend
```

### Stop Services

```bash
# Stop without removing data
docker compose down

# Stop and remove volumes (WARNING: deletes all data!)
docker compose down -v
```

### Update M'Eye

```bash
# Pull latest images
docker compose pull

# Recreate containers
docker compose up -d --force-recreate

# Run migrations if any
docker compose exec backend ./run-migrations
```

## Database Management

### Connect to PostgreSQL

```bash
# Using docker exec
docker compose exec postgres psql -U m_eye -d m_eye

# Or from host (if port exposed)
psql postgresql://m_eye:m_eye_password@localhost:5432/m_eye
```

### Backup Database

```bash
# Create backup
docker compose exec postgres pg_dump -U m_eye m_eye > backup.sql

# Restore from backup
docker compose exec -T postgres psql -U m_eye m_eye < backup.sql
```

### View Database Schema

```bash
docker compose exec postgres psql -U m_eye -d m_eye -c "\dt"
```

## Redis Management

### Connect to Redis

```bash
docker compose exec redis redis-cli
```

### Common Redis Commands

```bash
# View all keys
KEYS *

# Get reputation cache
GET reputation:email:user@example.com

# Clear cache
FLUSHDB

# View memory usage
INFO memory
```

## Health Checks

### Check Service Health

```bash
# API health endpoint
curl http://localhost:8000/api/v1/health

# PostgreSQL health
docker compose exec postgres pg_isready -U m_eye -d m_eye

# Redis health
docker compose exec redis redis-cli ping
```

### Docker Compose Health

```bash
docker compose ps
```

All services should show `healthy` status.

## Troubleshooting

### Backend Won't Start

**Symptoms**: Container exits immediately

**Solutions**:
1. Check logs: `docker compose logs backend`
2. Verify database is ready: `docker compose logs postgres`
3. Check environment variables in `.env`
4. Ensure ports 8000 and 2525 are not in use

### Database Connection Errors

**Symptoms**: "Connection refused" or "Authentication failed"

**Solutions**:
1. Verify PostgreSQL is running: `docker compose ps postgres`
2. Check DATABASE_URL in `.env`
3. Ensure network connectivity between containers
4. Restart PostgreSQL: `docker compose restart postgres`

### Port Conflicts

**Symptoms**: "Address already in use"

**Solutions**:
```bash
# Find process using port 8000
lsof -i :8000

# Change port in .env
API_PORT=8001
SMTP_PORT=2526

# Restart
docker compose down
docker compose up -d
```

### High Memory Usage

**Solutions**:
1. Limit container memory in `docker-compose.yml`:
   ```yaml
   services:
     backend:
       deploy:
         resources:
           limits:
             memory: 512M
   ```
2. Reduce RUST_LOG level to `warn` or `error`
3. Consider adding swap space

### Slow Performance

**Solutions**:
1. Check database query performance
2. Verify Redis is being used for caching
3. Increase PostgreSQL shared_buffers
4. Add database indexes if needed

## Production Deployment

### Reverse Proxy Configuration

Place M'Eye behind a reverse proxy for TLS termination:

#### nginx Example

```nginx
server {
    listen 443 ssl;
    server_name m-eye.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains";
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
}
```

#### Traefik Example

```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.m-eye.rule=Host(`m-eye.example.com`)"
  - "traefik.http.routers.m-eye.entrypoints=websecure"
  - "traefik.http.routers.m-eye.tls.certresolver=myresolver"
```

### Database Hardening

1. **Don't expose PostgreSQL to internet**
2. **Use strong passwords** (min 32 characters)
3. **Enable SSL connections**
4. **Regular backups** with offsite storage
5. **Monitor slow queries**

### Monitoring Setup

Recommended monitoring:

1. **Application Metrics**: Prometheus + Grafana
2. **Log Aggregation**: ELK Stack or Loki
3. **Alerting**: PagerDuty or Opsgenie
4. **Uptime Monitoring**: UptimeRobot or Pingdom

### Security Checklist

- [ ] Change all default passwords
- [ ] Generate strong JWT secret
- [ ] Enable HTTPS/TLS
- [ ] Configure firewall rules
- [ ] Set up regular backups
- [ ] Enable audit logging
- [ ] Configure rate limiting
- [ ] Review RBAC permissions
- [ ] Set up monitoring/alerting
- [ ] Document recovery procedures

## Development Setup

### Local Development

```bash
# Start only database and Redis
docker compose up -d postgres redis

# Run backend locally
cd backend
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features
```

### Database Migrations

Migrations are automatically applied on first startup. For manual migration:

```bash
docker compose exec postgres psql -U m_eye -d m_eye -f /docker-entrypoint-initdb.d/001_initial_schema.sql
```

### Seed Data

Default admin account is created automatically. To add more users:

```sql
INSERT INTO users (email, password_hash, role) 
VALUES ('user@example.com', '$argon2id$...', 'member');
```

## Next Steps

After installation:

1. **Change admin password** immediately
2. **Review security settings** in SECURITY.md
3. **Configure SMTP integration** with your mail server
4. **Set up monitoring** for production deployments
5. **Read the documentation** in docs/

## Support

- **Documentation**: See README.md and docs/
- **Issues**: GitHub Issues
- **Security**: See SECURITY.md
- **Community**: GitHub Discussions

---

For additional help, consult the main README.md or open an issue on GitHub.
