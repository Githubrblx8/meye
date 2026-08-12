export interface User {
  id: string;
  email: string;
  role: 'member' | 'trusted_reporter' | 'security_researcher' | 'moderator' | 'administrator';
  created_at: string;
}

export interface Reputation {
  identity: string;
  type: 'email' | 'domain' | 'ip' | 'url';
  status: 'SAFE' | 'UNKNOWN' | 'WATCH' | 'BLOCKED' | 'COMPROMISED';
  risk_score: number;
  confidence: number;
  reports_count: number;
  confirmed_incidents: number;
  first_seen: string;
  last_seen: string;
}

export interface Report {
  id: string;
  target: string;
  category: 'spam' | 'phishing' | 'malware' | 'credential_harvesting' | 'impersonation' | 'fraud' | 'compromised_account' | 'suspicious' | 'other';
  description: string;
  status: 'pending' | 'approved' | 'rejected' | 'confirmed';
  created_at: string;
  reporter_id: string;
}

export interface Evidence {
  id: string;
  type: 'email_headers' | 'url' | 'domain' | 'ip' | 'file_hash' | 'screenshot' | 'spf_result' | 'dkim_result' | 'dmarc_result' | 'dns_information' | 'sandbox_result' | 'threat_intelligence_reference' | 'analyst_report';
  value: string;
  source: string;
  submitted_by: string;
  confidence: number;
  created_at: string;
}

export interface Suggestion {
  id: string;
  target: string;
  suggested_status: 'SAFE' | 'WATCH' | 'BLOCKED' | 'COMPROMISED';
  reason: string;
  description: string;
  evidence: Evidence[];
  status: 'pending' | 'approved' | 'rejected';
  submitted_by: string;
  created_at: string;
}

export interface RiskAssessment {
  risk_score: number;
  decision: 'ALLOW' | 'WATCH' | 'BLOCK' | 'QUARANTINE';
  confidence: number;
  factors: RiskFactor[];
  recommendation: string;
}

export interface RiskFactor {
  name: string;
  contribution: number;
  weight: number;
  confidence: number;
  evidence: string;
}

export interface AuthTokens {
  access_token: string;
  token_type: string;
  expires_in: number;
  user: User;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
}
