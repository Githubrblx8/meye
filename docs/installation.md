# M'Eye Installation Guide

## Quick Start (Recommended)

The easiest way to install M'Eye is using the automated installation script.

### Prerequisites

- Docker 20+ installed
- Docker Compose 2+ installed
- Git (for cloning the repository)

### One-Command Installation

```bash
# Clone the repository
git clone https://github.com/your-org/m-eye.git
cd m-eye

# Run the installation script
./install.sh
```

The script will:
1. Verify Docker installation
2. Create a secure `.env` file
3. Generate random secrets
4. Start all services
5. Wait for services to be ready
6. Display access information

### Access M'Eye

After installation:

- **Web Interface**: http://localhost:3000
- **API**: http://localhost:8000
- **API Documentation**: http://localhost:8000/docs

### Default Credentials

```
Email: admin@m-eye.local
Password: ChangeMe123!
```

⚠️ **IMPORTANT**: Change the default password immediately!

---

## Manual Installation

If you prefer manual setup:

### Step 1: Clone Repository

```bash
git clone https://github.com/your-org/m-eye.git
cd m-eye
```

### Step 2: Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Generate secure secret key
SECRET_KEY=$(openssl rand -hex 32)
echo "SECRET_KEY=$SECRET_KEY" >> .env

# Generate secure database password
DB_PASSWORD=$(openssl rand -base64 24 | tr -d '=+/')
sed -i "s/change-me-secure-password/$DB_PASSWORD/" .env
```

### Step 3: Review Configuration

Edit `.env` file to customize:

```bash
# Application settings
ENVIRONMENT=production
LOG_LEVEL=INFO

# Admin account (change these!)
ADMIN_EMAIL=admin@your-domain.com
ADMIN_PASSWORD=YourSecurePassword123!

# Ports (if you need different ports)
API_PORT=8000
WEB_PORT=3000
SMTP_PORT=2525
```

### Step 4: Start Services

```bash
docker compose up -d
```

### Step 5: Verify Installation

```bash
# Check container status
docker compose ps

# View logs
docker compose logs -f

# Test API health
curl http://localhost:8000/api/v1/health
```

---

## Configuration Options

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SECRET_KEY` | JWT signing key | Auto-generated |
| `DATABASE_URL` | PostgreSQL connection | Set automatically |
| `REDIS_URL` | Redis connection | Set automatically |
| `ENVIRONMENT` | dev/staging/prod | `development` |
| `LOG_LEVEL` | Logging verbosity | `INFO` |
| `API_PORT` | API port | `8000` |
| `WEB_PORT` | Web UI port | `3000` |
| `SMTP_PORT` | SMTP gateway port | `2525` |

### Feature Flags

Enable/disable features in `.env`:

```bash
ENABLE_THREAT_INTEL=false
ENABLE_OPENSEARCH=false
ENABLE_ADVANCED_ANALYTICS=false
```

---

## Troubleshooting

### Containers Won't Start

```bash
# Check Docker daemon
sudo systemctl status docker

# Check Docker Compose version
docker compose version

# View error logs
docker compose logs
```

### Database Connection Issues

```bash
# Check if PostgreSQL is ready
docker compose exec db pg_isready -U meye

# Restart database
docker compose restart db

# View database logs
docker compose logs db
```

### Port Conflicts

If ports 8000, 3000, or 2525 are already in use:

1. Edit `.env` file
2. Change the port numbers
3. Restart services

```bash
API_PORT=8080
WEB_PORT=3080
SMTP_PORT=2580
```

### Permission Issues

```bash
# Fix ownership
sudo chown -R $USER:$USER .

# Ensure install script is executable
chmod +x install.sh
```

### Reset Everything

```bash
# Stop and remove all containers and volumes
docker compose down -v

# Remove generated files
rm .env

# Start fresh
./install.sh
```

---

## Production Deployment

For production environments:

### 1. Security Hardening

- Use strong passwords
- Enable HTTPS/TLS
- Configure firewall rules
- Restrict network access
- Enable audit logging

### 2. Backup Strategy

```bash
# Backup PostgreSQL
docker compose exec db pg_dump -U meye meye > backup.sql

# Backup Redis (if persistence enabled)
cp /var/lib/docker/volumes/m-eye_redis_data/_data/dump.rdb ./redis-backup.rdb
```

### 3. Monitoring

Set up monitoring for:
- Container health
- Database performance
- Disk space
- Memory usage
- API response times

### 4. Updates

```bash
# Pull latest images
docker compose pull

# Restart with new images
docker compose up -d

# Verify everything is working
docker compose ps
```

---

## Next Steps

After installation:

1. ✅ Change default admin password
2. ✅ Configure HTTPS for production
3. ✅ Set up regular backups
4. ✅ Review security settings
5. ✅ Invite team members
6. ✅ Configure email notifications (optional)
7. ✅ Set up threat intelligence providers (Phase 3)

For usage instructions, see the [README](../README.md).

For API documentation, visit http://localhost:8000/docs.
