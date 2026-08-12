"""
User model with RBAC support
"""
from sqlalchemy import Column, Integer, String, Boolean, DateTime
from sqlalchemy.sql import func
from ..core.database import Base


class User(Base):
    """User model with role-based access control"""
    
    __tablename__ = "users"
    
    id = Column(Integer, primary_key=True, index=True)
    email = Column(String(255), unique=True, index=True, nullable=False)
    name = Column(String(255), nullable=False)
    hashed_password = Column(String(255), nullable=False)
    role = Column(String(50), default="member", nullable=False)  # member, trusted_reporter, researcher, moderator, admin
    is_active = Column(Boolean, default=True, nullable=False)
    created_at = Column(DateTime(timezone=True), server_default=func.now())
    updated_at = Column(DateTime(timezone=True), onupdate=func.now())
    last_login = Column(DateTime(timezone=True), nullable=True)
    
    # Role weights for reputation system
    @property
    def reputation_weight(self) -> float:
        """Get the weight of this user's reports in reputation calculation"""
        weights = {
            "member": 1.0,
            "trusted_reporter": 2.0,
            "researcher": 3.0,
            "moderator": 4.0,
            "admin": 5.0
        }
        return weights.get(self.role, 1.0)
    
    @property
    def can_moderate(self) -> bool:
        """Check if user can moderate reports"""
        return self.role in ["moderator", "admin"]
    
    @property
    def can_publish_research(self) -> bool:
        """Check if user can publish research reports"""
        return self.role in ["researcher", "moderator", "admin"]
