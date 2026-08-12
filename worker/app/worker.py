"""
M'Eye Worker - SMTP Gateway and Async Jobs
"""
import asyncio
import logging
import os
from datetime import datetime
from typing import Optional, Dict, Any

# Configure logging
logging.basicConfig(
    level=os.getenv('LOG_LEVEL', 'INFO'),
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class SMTPGateway:
    """Simple SMTP Gateway for email analysis"""
    
    def __init__(self, host: str = "0.0.0.0", port: int = 2525):
        self.host = host
        self.port = port
        self.stats = {
            "emails_processed": 0,
            "emails_allowed": 0,
            "emails_blocked": 0,
            "emails_watched": 0
        }
    
    async def analyze_email(self, email_data: Dict[str, Any]) -> Dict[str, Any]:
        """
        Analyze an email and return decision
        
        Returns dict with:
        - decision: ALLOW, WATCH, or BLOCK
        - risk_score: 0-100
        - factors: list of risk factors
        """
        factors = []
        risk_score = 0
        
        # Check SPF
        if email_data.get('spf_result') == 'fail':
            risk_score += 25
            factors.append("+25 SPF failure")
        
        # Check DKIM
        if email_data.get('dkim_result') == 'fail':
            risk_score += 20
            factors.append("+20 DKIM failure")
        
        # Check DMARC
        if email_data.get('dmarc_result') == 'fail':
            risk_score += 25
            factors.append("+25 DMARC failure")
        
        # Cap risk score at 100
        risk_score = min(100, risk_score)
        
        # Determine decision
        if risk_score >= 60:
            decision = "BLOCK"
            self.stats["emails_blocked"] += 1
        elif risk_score >= 40:
            decision = "WATCH"
            self.stats["emails_watched"] += 1
        else:
            decision = "ALLOW"
            self.stats["emails_allowed"] += 1
        
        self.stats["emails_processed"] += 1
        
        return {
            "decision": decision,
            "risk_score": risk_score,
            "factors": factors,
            "timestamp": datetime.utcnow().isoformat()
        }
    
    async def start_server(self):
        """Start the SMTP server (placeholder for MVP)"""
        logger.info(f"SMTP Gateway would start on {self.host}:{self.port}")
        logger.info("For MVP, SMTP gateway is simulated")
        # In production, this would use aiosmtplib or similar
        # For now, it's a placeholder that logs what would happen


async def main():
    """Main worker entry point"""
    logger.info("Starting M'Eye Worker...")
    
    # Initialize SMTP gateway
    smtp_host = os.getenv('SMTP_HOST', '0.0.0.0')
    smtp_port = int(os.getenv('SMTP_PORT', '2525'))
    
    gateway = SMTPGateway(host=smtp_host, port=smtp_port)
    
    # Start SMTP server (simulated for MVP)
    await gateway.start_server()
    
    # Keep worker running
    while True:
        await asyncio.sleep(3600)
        logger.info(f"Worker stats: {gateway.stats}")


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Worker shutting down...")
