#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo "╔══════════════════════════════════╗"
echo "║            M'Eye                 ║"
echo "║      Installation Script         ║"
echo "╚══════════════════════════════════╝"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker first.${NC}"
    echo "Visit: https://docs.docker.com/get-docker/"
    exit 1
fi

echo -e "${GREEN}✓${NC} Docker is installed"

# Check if Docker Compose is available
if ! docker compose version &> /dev/null; then
    echo -e "${RED}❌ Docker Compose is not installed.${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Docker Compose is available"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "${BLUE}📝${NC} Creating .env file from .env.example..."
    cp .env.example .env
    
    # Generate a random JWT secret
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || head -c 32 /dev/urandom | base64)
    sed -i.bak "s/JWT_SECRET=.*/JWT_SECRET=$JWT_SECRET/" .env
    rm -f .env.bak
    
    # Generate a random database password
    DB_PASSWORD=$(openssl rand -base64 16 2>/dev/null || head -c 16 /dev/urandom | base64 | tr -d '/=')
    sed -i.bak "s/POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=$DB_PASSWORD/" .env
    rm -f .env.bak
    
    echo -e "${GREEN}✓${NC} .env file created with secure secrets"
else
    echo -e "${YELLOW}⚠${NC} .env file already exists, skipping creation"
fi

echo ""
echo -e "${BLUE}🐳${NC} Starting Docker containers..."
docker compose up -d

echo ""
echo -e "${BLUE}⏳${NC} Waiting for PostgreSQL to be ready..."
sleep 10

# Wait for PostgreSQL to be healthy
MAX_RETRIES=30
RETRY_COUNT=0
while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if docker compose exec -T postgres pg_isready -U m_eye -d m_eye &> /dev/null; then
        echo -e "${GREEN}✓${NC} PostgreSQL is ready"
        break
    fi
    RETRY_COUNT=$((RETRY_COUNT + 1))
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${RED}❌ PostgreSQL failed to start. Check logs with: docker compose logs postgres${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}⏳${NC} Waiting for backend to be ready..."
MAX_RETRIES=30
RETRY_COUNT=0
while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s http://localhost:8000/api/v1/health &> /dev/null; then
        echo -e "${GREEN}✓${NC} Backend API is ready"
        break
    fi
    RETRY_COUNT=$((RETRY_COUNT + 1))
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${YELLOW}⚠${NC} Backend API is still starting. It should be ready soon."
fi

echo ""
echo "╔══════════════════════════════════╗"
echo "║            M'Eye                 ║"
echo "║      Installation Complete       ║"
echo "╚══════════════════════════════════╝"
echo ""
echo -e "${GREEN}✓${NC} Web Interface: http://localhost:3000"
echo -e "${GREEN}✓${NC} API:           http://localhost:8000"
echo -e "${GREEN}✓${NC} API Docs:      http://localhost:8000/docs"
echo -e "${GREEN}✓${NC} SMTP Gateway:  localhost:2525"
echo ""
echo "Admin account:"
echo "  Email:    admin@m-eye.local"
echo "  Password: ChangeMe123!"
echo ""
echo -e "${YELLOW}⚠${NC} Remember to change the default password!"
echo ""
echo -e "View logs:     ${BLUE}docker compose logs -f${NC}"
echo -e "Stop services: ${BLUE}docker compose down${NC}"
echo ""
