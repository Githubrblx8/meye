-- M'Eye Database Schema
-- PostgreSQL 15+

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- User roles enum
CREATE TYPE user_role AS ENUM (
    'member',
    'trusted_reporter',
    'security_researcher',
    'moderator',
    'administrator'
);

-- Identity types enum
CREATE TYPE identity_type AS ENUM (
    'email',
    'domain',
    'ip',
    'url'
);

-- Reputation status enum
CREATE TYPE reputation_status AS ENUM (
    'SAFE',
    'UNKNOWN',
    'WATCH',
    'BLOCKED',
    'COMPROMISED'
);

-- Report categories enum
CREATE TYPE report_category AS ENUM (
    'spam',
    'phishing',
    'malware',
    'credential_harvesting',
    'impersonation',
    'fraud',
    'compromised_account',
    'suspicious',
    'other'
);

-- Report status enum
CREATE TYPE report_status AS ENUM (
    'pending',
    'under_review',
    'confirmed',
    'rejected'
);

-- Evidence types enum
CREATE TYPE evidence_type AS ENUM (
    'email_headers',
    'url',
    'domain',
    'ip',
    'file_hash',
    'screenshot',
    'spf_result',
    'dkim_result',
    'dmarc_result',
    'dns_information',
    'sandbox_result',
    'threat_intel_reference',
    'analyst_report'
);

-- Suggestion status enum
CREATE TYPE suggestion_status AS ENUM (
    'pending',
    'approved',
    'rejected'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'member',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- Reputations table
CREATE TABLE reputations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identity VARCHAR(512) NOT NULL,
    identity_type identity_type NOT NULL,
    status reputation_status NOT NULL DEFAULT 'UNKNOWN',
    risk_score INTEGER NOT NULL DEFAULT 0 CHECK (risk_score >= 0 AND risk_score <= 100),
    confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    reports_count INTEGER NOT NULL DEFAULT 0,
    confirmed_incidents INTEGER NOT NULL DEFAULT 0,
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(identity, identity_type)
);

CREATE INDEX idx_reputations_identity ON reputations(identity);
CREATE INDEX idx_reputations_type ON reputations(identity_type);
CREATE INDEX idx_reputations_status ON reputations(status);
CREATE INDEX idx_reputations_risk_score ON reputations(risk_score);

-- Reports table
CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    target VARCHAR(512) NOT NULL,
    target_type identity_type NOT NULL,
    category report_category NOT NULL,
    description TEXT NOT NULL,
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status report_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reports_target ON reports(target);
CREATE INDEX idx_reports_category ON reports(category);
CREATE INDEX idx_reports_status ON reports(status);
CREATE INDEX idx_reports_reporter ON reports(reporter_id);

-- Evidence table
CREATE TABLE evidence (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    evidence_type evidence_type NOT NULL,
    value TEXT NOT NULL,
    source VARCHAR(255) NOT NULL,
    submitted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_evidence_report ON evidence(report_id);
CREATE INDEX idx_evidence_submitted_by ON evidence(submitted_by);

-- Suggestions table
CREATE TABLE suggestions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    target VARCHAR(512) NOT NULL,
    target_type identity_type NOT NULL,
    suggested_status reputation_status NOT NULL,
    reason VARCHAR(255) NOT NULL,
    description TEXT,
    submitted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status suggestion_status NOT NULL DEFAULT 'pending',
    moderator_id UUID REFERENCES users(id) ON DELETE SET NULL,
    moderated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_suggestions_target ON suggestions(target);
CREATE INDEX idx_suggestions_status ON suggestions(status);
CREATE INDEX idx_suggestions_submitted_by ON suggestions(submitted_by);

-- Reputation history table (audit trail)
CREATE TABLE reputation_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    reputation_id UUID NOT NULL REFERENCES reputations(id) ON DELETE CASCADE,
    old_status reputation_status,
    new_status reputation_status NOT NULL,
    old_risk_score INTEGER,
    new_risk_score INTEGER NOT NULL,
    reason TEXT,
    changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reputation_history_reputation ON reputation_history(reputation_id);
CREATE INDEX idx_reputation_history_created_at ON reputation_history(created_at);

-- SMTP logs table
CREATE TABLE smtp_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    direction VARCHAR(10) NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    sender_email VARCHAR(255) NOT NULL,
    recipient_email VARCHAR(255) NOT NULL,
    sender_ip INET,
    spf_result VARCHAR(50),
    dkim_result VARCHAR(50),
    dmarc_result VARCHAR(50),
    risk_score INTEGER CHECK (risk_score >= 0 AND risk_score <= 100),
    decision VARCHAR(20) CHECK (decision IN ('ALLOW', 'WATCH', 'BLOCK')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_smtp_logs_sender ON smtp_logs(sender_email);
CREATE INDEX idx_smtp_logs_recipient ON smtp_logs(recipient_email);
CREATE INDEX idx_smtp_logs_created_at ON smtp_logs(created_at);

-- Insert default admin user (password: ChangeMe123!)
-- Hash generated with argon2id
INSERT INTO users (email, password_hash, role) VALUES
('admin@m-eye.local', '$argon2id$v=19$m=65536,t=3,p=4$YWRtaW5AbS1leWUubG9jYWw$KqPz8vN9xJxL5qR3mT6wU2hF8nD4cB1aE7gH9iJ0kL2', 'administrator')
ON CONFLICT (email) DO NOTHING;
