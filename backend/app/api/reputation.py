"""
Reputation API endpoints
"""
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from typing import Optional
import re

from ..core.database import get_db
from ..models.reputation import Reputation, IdentityType, ReputationStatus
from ..models.report import Report

router = APIRouter()


def validate_email(email: str) -> bool:
    """Validate email format"""
    pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
    return re.match(pattern, email) is not None


def validate_domain(domain: str) -> bool:
    """Validate domain format"""
    pattern = r'^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+$'
    return re.match(pattern, domain) is not None


def validate_ip(ip: str) -> bool:
    """Validate IPv4 address format"""
    pattern = r'^(\d{1,3}\.){3}\d{1,3}$'
    if not re.match(pattern, ip):
        return False
    
    # Check each octet
    parts = ip.split('.')
    return all(0 <= int(part) <= 255 for part in parts)


def detect_identity_type(identity: str) -> Optional[IdentityType]:
    """Detect the type of identity from the string"""
    if validate_email(identity):
        return IdentityType.EMAIL
    elif validate_domain(identity):
        return IdentityType.DOMAIN
    elif validate_ip(identity):
        return IdentityType.IP
    elif identity.startswith('http://') or identity.startswith('https://'):
        return IdentityType.URL
    return None


@router.get("/email/{email:path}")
async def get_email_reputation(email: str, db: Session = Depends(get_db)):
    """Get reputation for an email address"""
    if not validate_email(email):
        raise HTTPException(status_code=400, detail="Invalid email format")
    
    reputation = db.query(Reputation).filter(
        Reputation.identity == email.lower(),
        Reputation.identity_type == IdentityType.EMAIL
    ).first()
    
    if not reputation:
        # Return UNKNOWN status for new emails
        return {
            "identity": email,
            "type": "email",
            "status": "UNKNOWN",
            "risk_score": 0,
            "confidence": 0.0,
            "reports_count": 0,
            "confirmed_incidents": 0,
            "message": "No reputation data available"
        }
    
    return reputation.to_dict()


@router.get("/domain/{domain:path}")
async def get_domain_reputation(domain: str, db: Session = Depends(get_db)):
    """Get reputation for a domain"""
    if not validate_domain(domain):
        raise HTTPException(status_code=400, detail="Invalid domain format")
    
    reputation = db.query(Reputation).filter(
        Reputation.identity == domain.lower(),
        Reputation.identity_type == IdentityType.DOMAIN
    ).first()
    
    if not reputation:
        return {
            "identity": domain,
            "type": "domain",
            "status": "UNKNOWN",
            "risk_score": 0,
            "confidence": 0.0,
            "reports_count": 0,
            "confirmed_incidents": 0,
            "message": "No reputation data available"
        }
    
    return reputation.to_dict()


@router.get("/ip/{ip_address:path}")
async def get_ip_reputation(ip_address: str, db: Session = Depends(get_db)):
    """Get reputation for an IP address"""
    if not validate_ip(ip_address):
        raise HTTPException(status_code=400, detail="Invalid IP address format")
    
    reputation = db.query(Reputation).filter(
        Reputation.identity == ip_address,
        Reputation.identity_type == IdentityType.IP
    ).first()
    
    if not reputation:
        return {
            "identity": ip_address,
            "type": "ip",
            "status": "UNKNOWN",
            "risk_score": 0,
            "confidence": 0.0,
            "reports_count": 0,
            "confirmed_incidents": 0,
            "message": "No reputation data available"
        }
    
    return reputation.to_dict()


@router.post("/{identity_type:path}/recalculate")
async def recalculate_reputation(
    identity_type: str,
    identity_value: str,
    db: Session = Depends(get_db)
):
    """Recalculate reputation score based on reports"""
    # This would trigger the reputation engine
    # For MVP, we'll just return the current state
    
    id_type = detect_identity_type(identity_value)
    if not id_type:
        raise HTTPException(status_code=400, detail="Invalid identity format")
    
    reputation = db.query(Reputation).filter(
        Reputation.identity == identity_value.lower(),
        Reputation.identity_type == id_type
    ).first()
    
    if not reputation:
        raise HTTPException(status_code=404, detail="Identity not found")
    
    # Simple reputation calculation for MVP
    reports = db.query(Report).filter(
        Report.target == identity_value.lower(),
        Report.status == "confirmed"
    ).all()
    
    # Calculate risk score based on confirmed reports
    risk_score = min(100, len(reports) * 15)  # Each confirmed report adds 15 points
    
    # Determine status based on risk score
    if risk_score >= 80:
        new_status = ReputationStatus.COMPROMISED
    elif risk_score >= 60:
        new_status = ReputationStatus.BLOCKED
    elif risk_score >= 40:
        new_status = ReputationStatus.WATCH
    elif risk_score >= 20:
        new_status = ReputationStatus.UNKNOWN
    else:
        new_status = ReputationStatus.SAFE
    
    # Update reputation
    old_status = reputation.status
    reputation.status = new_status
    reputation.risk_score = risk_score
    reputation.confidence = min(1.0, len(reports) / 5.0)  # Confidence increases with more reports
    reputation.reports_count = len(reports)
    
    db.commit()
    db.refresh(reputation)
    
    return {
        "message": "Reputation recalculated",
        "previous_status": old_status.value,
        "new_status": new_status.value,
        "new_risk_score": risk_score,
        "reputation": reputation.to_dict()
    }
