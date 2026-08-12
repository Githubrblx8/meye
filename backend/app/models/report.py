"""
Report, Evidence, and Suggestion models
"""
from sqlalchemy import Column, Integer, String, Float, DateTime, ForeignKey, Text, Enum as SQLEnum, Boolean
from sqlalchemy.sql import func
from sqlalchemy.orm import relationship
import enum
from ..core.database import Base


class ReportCategory(str, enum.Enum):
    """Categories for reports"""
    SPAM = "spam"
    PHISHING = "phishing"
    MALWARE = "malware"
    CREDENTIAL_HARVESTING = "credential_harvesting"
    IMPERSONATION = "impersonation"
    FRAUD = "fraud"
    COMPROMISED_ACCOUNT = "compromised_account"
    SUSPICIOUS = "suspicious"
    OTHER = "other"


class ReportStatus(str, enum.Enum):
    """Status of a report"""
    PENDING = "pending"
    UNDER_REVIEW = "under_review"
    CONFIRMED = "confirmed"
    REJECTED = "rejected"
    DUPLICATE = "duplicate"


class EvidenceType(str, enum.Enum):
    """Types of evidence that can be submitted"""
    EMAIL_HEADERS = "email_headers"
    URL = "url"
    DOMAIN = "domain"
    IP = "ip"
    FILE_HASH = "file_hash"
    SCREENSHOT = "screenshot"
    SPF_RESULT = "spf_result"
    DKIM_RESULT = "dkim_result"
    DMARC_RESULT = "dmarc_result"
    DNS_INFORMATION = "dns_information"
    SANDBOX_RESULT = "sandbox_result"
    THREAT_INTEL_REFERENCE = "threat_intel_reference"
    ANALYST_REPORT = "analyst_report"
    OTHER = "other"


class SuggestedStatus(str, enum.Enum):
    """Suggested status values"""
    SAFE = "SAFE"
    WATCH = "WATCH"
    BLOCKED = "BLOCKED"
    COMPROMISED = "COMPROMISED"


class Report(Base):
    """Community report about an identity"""
    
    __tablename__ = "reports"
    
    id = Column(Integer, primary_key=True, index=True)
    target = Column(String(512), index=True, nullable=False)  # email, domain, IP, or URL
    reporter_id = Column(Integer, ForeignKey("users.id"), nullable=False)
    
    # Report details
    category = Column(SQLEnum(ReportCategory), nullable=False)
    description = Column(Text, nullable=False)
    
    # Status
    status = Column(SQLEnum(ReportStatus), default=ReportStatus.PENDING, nullable=False)
    
    # Review
    reviewed_by = Column(Integer, ForeignKey("users.id"), nullable=True)
    reviewed_at = Column(DateTime(timezone=True), nullable=True)
    review_notes = Column(Text, nullable=True)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())
    
    # Relationships
    reporter = relationship("User", foreign_keys=[reporter_id])
    reviewer = relationship("User", foreign_keys=[reviewed_by])
    reputation = relationship("Reputation", back_populates="reports")
    evidences = relationship("Evidence", back_populates="report", cascade="all, delete-orphan")
    
    def to_dict(self) -> dict:
        """Convert to dictionary for API response"""
        return {
            "id": self.id,
            "target": self.target,
            "category": self.category.value,
            "description": self.description,
            "status": self.status.value,
            "reporter_id": self.reporter_id,
            "created_at": self.created_at.isoformat() if self.created_at else None,
            "evidences": [e.to_dict() for e in self.evidences]
        }


class Evidence(Base):
    """Evidence attached to a report"""
    
    __tablename__ = "evidence"
    
    id = Column(Integer, primary_key=True, index=True)
    report_id = Column(Integer, ForeignKey("reports.id"), nullable=False)
    submitted_by = Column(Integer, ForeignKey("users.id"), nullable=False)
    
    # Evidence details
    evidence_type = Column(SQLEnum(EvidenceType), nullable=False)
    value = Column(Text, nullable=False)  # The actual evidence (headers, URL, hash, etc.)
    source = Column(String(255), nullable=True)  # Where the evidence came from
    
    # Confidence score (0.0-1.0)
    confidence = Column(Float, default=0.5, nullable=False)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    
    # Relationships
    report = relationship("Report", back_populates="evidences")
    submitter = relationship("User")
    
    def to_dict(self) -> dict:
        """Convert to dictionary for API response"""
        return {
            "id": self.id,
            "type": self.evidence_type.value,
            "value": self.value,
            "source": self.source,
            "confidence": self.confidence,
            "submitted_by": self.submitted_by,
            "created_at": self.created_at.isoformat() if self.created_at else None
        }


class Suggestion(Base):
    """User suggestion for changing reputation status"""
    
    __tablename__ = "suggestions"
    
    id = Column(Integer, primary_key=True, index=True)
    target = Column(String(512), index=True, nullable=False)  # email, domain, IP, or URL
    submitted_by = Column(Integer, ForeignKey("users.id"), nullable=False)
    
    # Suggestion details
    suggested_status = Column(SQLEnum(SuggestedStatus), nullable=False)
    reason = Column(String(255), nullable=False)
    description = Column(Text, nullable=False)
    
    # Status
    status = Column(String(50), default="pending", nullable=False)  # pending, approved, rejected
    reviewed_by = Column(Integer, ForeignKey("users.id"), nullable=True)
    
    # Timestamps
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    reviewed_at = Column(DateTime(timezone=True), nullable=True)
    
    # Relationships
    submitter = relationship("User", foreign_keys=[submitted_by])
    reviewer = relationship("User", foreign_keys=[reviewed_by])
    
    def to_dict(self) -> dict:
        """Convert to dictionary for API response"""
        return {
            "id": self.id,
            "target": self.target,
            "suggested_status": self.suggested_status.value,
            "reason": self.reason,
            "description": self.description,
            "status": self.status,
            "submitted_by": self.submitted_by,
            "created_at": self.created_at.isoformat() if self.created_at else None
        }
