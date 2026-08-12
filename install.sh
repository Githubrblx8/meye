#!/bin/bash

# M'Eye Installation Script
# This script automates the setup process

set -e

echo "╔══════════════════════════════════╗"
echo "║         M'Eye Installer          ║"
echo "╚══════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker first.${NC}"
    echo "Visit: https://docs.docker.com/get-docker/"
    exit 1
fi

echo -e "${GREEN}✓${NC} Docker is installed"

# Check if Docker Compose is installed
if ! command -v docker compose &> /dev/null && ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}❌ Docker Compose is not installed. Please install Docker Compose first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Docker Compose is installed"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "${YELLOW}⚙️${NC} Creating .env file..."
    cp .env.example .env
    
    # Generate a secure secret key
    SECRET_KEY=$(openssl rand -hex 32)
    sed -i "s/your-secret-key-change-in-production/$SECRET_KEY/" .env
    
    # Generate a secure database password
    DB_PASSWORD=$(openssl rand -base64 24 | tr -d '=+/')
    sed -i "s/change-me-secure-password/$DB_PASSWORD/" .env
    
    echo -e "${GREEN}✓${NC} .env file created with secure secrets"
else
    echo -e "${GREEN}✓${NC} .env file already exists"
fi

echo ""
echo -e "${YELLOW}📦${NC} Starting Docker containers..."

# Start containers
docker compose up -d

echo ""
echo -e "${YELLOW}⏳${NC} Waiting for services to be ready..."

# Wait for PostgreSQL to be ready
MAX_ATTEMPTS=30
ATTEMPT=0
while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    if docker compose exec -T db pg_isready -U meye > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} PostgreSQL is ready"
        break
    fi
    ATTEMPT=$((ATTEMPT + 1))
    sleep 2
done

if [ $ATTEMPT -eq $MAX_ATTEMPTS ]; then
    echo -e "${RED}❌ PostgreSQL failed to start. Check logs with: docker compose logs db${NC}"
    exit 1
fi

# Wait for backend to be ready
ATTEMPT=0
while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    if curl -s http://localhost:8000/api/v1/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} Backend API is ready"
        break
    fi
    ATTEMPT=$((ATTEMPT + 1))
    sleep 2
done

if [ $ATTEMPT -eq $MAX_ATTEMPTS ]; then
    echo -e "${YELLOW}⚠️${NC} Backend API is taking longer to start. Check logs with: docker compose logs backend"
fi

echo ""
echo "╔══════════════════════════════════╗"
echo "║       Installation Complete      ║"
echo "╚══════════════════════════════════╝"
echo ""
echo -e "${GREEN}Web Interface:${NC}  http://localhost:3000"
echo -e "${GREEN}API:${NC}            http://localhost:8000"
echo -e "${GREEN}API Docs:${NC}       http://localhost:8000/docs"
echo ""
echo -e "${YELLOW}Default Admin Account:${NC}"
echo "  Email: admin@m-eye.local"
echo "  Password: ChangeMe123!"
echo ""
echo -e "${RED}⚠️  IMPORTANT: Change the default admin password immediately!${NC}"
echo ""
echo "Useful commands:"
echo "  docker compose logs -f     # View logs"
echo "  docker compose down        # Stop all services"
echo "  docker compose restart     # Restart services"
echo ""
