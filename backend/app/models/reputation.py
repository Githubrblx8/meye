"""
Reputation models for email, domain, IP, and URL identities
"""
from sqlalchemy import Column, Integer, String, Float, DateTime, ForeignKey, Text, Enum as SQLEnum
from sqlalchemy.sql import func
from sqlalchemy.orm import relationship
import enum
from ..core.database import Base


class IdentityType(str, enum.Enum):
    """Type of identity being tracked"""
    EMAIL = "email"
    DOMAIN = "domain"
    IP = "ip"
    URL = "url"


class ReputationStatus(str, enum.Enum):
    """Reputation status values"""
    SAFE = "SAFE"           # 🟢 Reliable
    UNKNOWN = "UNKNOWN"     # ⚪ No sufficient information
    WATCH = "WATCH"         # 🟠 Suspicious / monitor
    BLOCKED = "BLOCKED"     # 🔴 Blocked
    COMPROMISED = "COMPROMISED"  # 🟣 Compromised


class Reputation(Base):
    """Reputation score for an identity (email, domain, IP, URL)"""
    
    __tablename__ = "reputations"
    
    id = Column(Integer, primary_key=True, index=True)
    identity = Column(String(512), unique=True, index=True, nullable=False)  # email, domain, IP, or URL
    identity_type = Column(SQLEnum(IdentityType), nullable=False, index=True)
    
    # Current status
    status = Column(SQLEnum(ReputationStatus), default=ReputationStatus.UNKNOWN, nullable=False)
    risk_score = Column(Integer, default=0, nullable=False)  # 0-100
    confidence = Column(Float, default=0.0, nullable=False)  # 0.0-1.0
    
    # Statistics
    reports_count = Column(Integer, default=0, nullable=False)
    confirmed_incidents = Column(Integer, default=0, nullable=False)
    
    # Timestamps
    first_seen = Column(DateTime(timezone=True), server_default=func.now())
    last_seen = Column(DateTime(timezone=True), onupdate=func.now())
    last_updated = Column(DateTime(timezone=True), server_default=func.now())
    
    # Risk factors (JSON-like text for explainability)
    risk_factors = Column(Text, nullable=True)  # Stores explanation of risk score
    
    # Relationships
    history = relationship("ReputationHistory", back_populates="reputation", cascade="all, delete-orphan")
    reports = relationship("Report", back_populates="reputation", cascade="all, delete-orphan")
    
    def to_dict(self) -> dict:
        """Convert to dictionary for API response"""
        return {
            "identity": self.identity,
            "type": self.identity_type.value,
            "status": self.status.value,
            "risk_score": self.risk_score,
            "confidence": self.confidence,
            "reports_count": self.reports_count,
            "confirmed_incidents": self.confirmed_incidents,
            "first_seen": self.first_seen.isoformat() if self.first_seen else None,
            "last_seen": self.last_seen.isoformat() if self.last_seen else None,
            "risk_factors": self.parse_risk_factors()
        }
    
    def parse_risk_factors(self) -> list:
        """Parse risk factors from text to list"""
        if not self.risk_factors:
            return []
        try:
            import json
            return json.loads(self.risk_factors)
        except:
            return [{"factor": self.risk_factors}]


class ReputationHistory(Base):
    """Historical record of reputation changes"""
    
    __tablename__ = "reputation_history"
    
    id = Column(Integer, primary_key=True, index=True)
    reputation_id = Column(Integer, ForeignKey("reputations.id"), nullable=False)
    
    # Previous values
    previous_status = Column(SQLEnum(ReputationStatus), nullable=False)
    previous_risk_score = Column(Integer, nullable=False)
    previous_confidence = Column(Float, nullable=False)
    
    # New values
    new_status = Column(SQLEnum(ReputationStatus), nullable=False)
    new_risk_score = Column(Integer, nullable=False)
    new_confidence = Column(Float, nullable=False)
    
    # Reason for change
    reason = Column(Text, nullable=True)
    changed_by = Column(String(255), nullable=True)  # User ID or "system" or "engine"
    
    # Timestamp
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    
    # Relationships
    reputation = relationship("Reputation", back_populates="history")
