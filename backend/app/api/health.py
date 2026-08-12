"""
Health check endpoints
"""
from fastapi import APIRouter, Depends
from sqlalchemy.orm import Session
from ..core.database import get_db
from datetime import datetime

router = APIRouter()


@router.get("/health")
async def health_check(db: Session = Depends(get_db)):
    """Check if the service is healthy"""
    try:
        # Check database connection
        db.execute("SELECT 1")
        
        return {
            "status": "healthy",
            "timestamp": datetime.utcnow().isoformat(),
            "services": {
                "database": "connected",
                "api": "running"
            }
        }
    except Exception as e:
        return {
            "status": "unhealthy",
            "timestamp": datetime.utcnow().isoformat(),
            "error": str(e)
        }


@router.get("/ready")
async def readiness_check():
    """Check if the service is ready to accept traffic"""
    return {
        "status": "ready",
        "timestamp": datetime.utcnow().isoformat()
    }
