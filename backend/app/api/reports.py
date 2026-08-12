"""
Reports API endpoints
"""
from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from typing import List, Optional
from pydantic import BaseModel, EmailStr

from ..core.database import get_db
from ..api.auth import get_current_user
from ..models.user import User
from ..models.report import Report, Evidence, Suggestion, ReportCategory, ReportStatus, EvidenceType, SuggestedStatus
from ..models.reputation import Reputation, IdentityType

router = APIRouter()


class ReportCreate(BaseModel):
    target: str
    category: ReportCategory
    description: str
    evidence: Optional[List[dict]] = None


class ReportResponse(BaseModel):
    id: int
    target: str
    category: str
    description: str
    status: str
    created_at: str
    
    class Config:
        from_attributes = True


class EvidenceCreate(BaseModel):
    evidence_type: EvidenceType
    value: str
    source: Optional[str] = None
    confidence: float = 0.5


class SuggestionCreate(BaseModel):
    target: str
    suggested_status: SuggestedStatus
    reason: str
    description: str


@router.post("/", response_model=ReportResponse)
async def create_report(
    report_data: ReportCreate,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Submit a new report"""
    # Create the report
    new_report = Report(
        target=report_data.target.lower(),
        reporter_id=current_user.id,
        category=report_data.category,
        description=report_data.description,
        status=ReportStatus.PENDING
    )
    
    db.add(new_report)
    db.commit()
    db.refresh(new_report)
    
    # Add evidence if provided
    if report_data.evidence:
        for ev in report_data.evidence:
            evidence = Evidence(
                report_id=new_report.id,
                submitted_by=current_user.id,
                evidence_type=EvidenceType(ev.get("type", "other")),
                value=ev.get("value", ""),
                source=ev.get("source"),
                confidence=ev.get("confidence", 0.5)
            )
            db.add(evidence)
        
        db.commit()
    
    # Update reputation reports count
    reputation = db.query(Reputation).filter(
        Reputation.identity == report_data.target.lower()
    ).first()
    
    if reputation:
        reputation.reports_count += 1
        db.commit()
    
    return new_report


@router.get("/", response_model=List[ReportResponse])
async def get_reports(
    skip: int = 0,
    limit: int = 50,
    status_filter: Optional[str] = None,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Get reports (filtered by role)"""
    query = db.query(Report)
    
    # Non-moderators can only see their own reports or confirmed ones
    if not current_user.can_moderate:
        query = query.filter(
            (Report.reporter_id == current_user.id) | 
            (Report.status == ReportStatus.CONFIRMED)
        )
    
    if status_filter:
        try:
            status_enum = ReportStatus(status_filter)
            query = query.filter(Report.status == status_enum)
        except ValueError:
            pass
    
    reports = query.offset(skip).limit(limit).all()
    return reports


@router.get("/{report_id}")
async def get_report(
    report_id: int,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Get a specific report"""
    report = db.query(Report).filter(Report.id == report_id).first()
    
    if not report:
        raise HTTPException(status_code=404, detail="Report not found")
    
    # Check permissions
    if not current_user.can_moderate and report.reporter_id != current_user.id and report.status != ReportStatus.CONFIRMED:
        raise HTTPException(status_code=403, detail="Not authorized to view this report")
    
    return report.to_dict()


@router.post("/evidence")
async def submit_evidence(
    evidence_data: EvidenceCreate,
    report_id: int,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Submit evidence for a report"""
    report = db.query(Report).filter(Report.id == report_id).first()
    
    if not report:
        raise HTTPException(status_code=404, detail="Report not found")
    
    evidence = Evidence(
        report_id=report_id,
        submitted_by=current_user.id,
        evidence_type=evidence_data.evidence_type,
        value=evidence_data.value,
        source=evidence_data.source,
        confidence=evidence_data.confidence
    )
    
    db.add(evidence)
    db.commit()
    db.refresh(evidence)
    
    return {"message": "Evidence submitted", "evidence": evidence.to_dict()}


@router.post("/suggestion")
async def submit_suggestion(
    suggestion_data: SuggestionCreate,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Submit a suggestion for changing reputation status"""
    suggestion = Suggestion(
        target=suggestion_data.target.lower(),
        submitted_by=current_user.id,
        suggested_status=suggestion_data.suggested_status,
        reason=suggestion_data.reason,
        description=suggestion_data.description
    )
    
    db.add(suggestion)
    db.commit()
    db.refresh(suggestion)
    
    return {"message": "Suggestion submitted", "suggestion": suggestion.to_dict()}


@router.get("/suggestions")
async def get_suggestions(
    skip: int = 0,
    limit: int = 50,
    current_user: User = Depends(get_current_user),
    db: Session = Depends(get_db)
):
    """Get pending suggestions (moderators only)"""
    if not current_user.can_moderate:
        raise HTTPException(status_code=403, detail="Moderator access required")
    
    suggestions = db.query(Suggestion).filter(
        Suggestion.status == "pending"
    ).offset(skip).limit(limit).all()
    
    return [s.to_dict() for s in suggestions]
