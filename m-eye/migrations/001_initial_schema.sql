-- M'Eye Database Schema
-- Migration: 001_initial_schema

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Reputations table
CREATE TABLE reputations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identity_type VARCHAR(50) NOT NULL,
    identity_value VARCHAR(512) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'UNKNOWN',
    risk_score INTEGER NOT NULL DEFAULT 0,
    confidence DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    first_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reports_count INTEGER NOT NULL DEFAULT 0,
    confirmed_incidents INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_reputations_identity ON reputations(identity_type, identity_value);
CREATE INDEX idx_reputations_status ON reputations(status);
CREATE INDEX idx_reputations_risk_score ON reputations(risk_score);
CREATE INDEX idx_reputations_last_seen ON reputations(last_seen);

-- Reports table
CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type VARCHAR(50) NOT NULL,
    target_value VARCHAR(512) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reports_reporter_id ON reports(reporter_id);
CREATE INDEX idx_reports_status ON reports(status);
CREATE INDEX idx_reports_category ON reports(category);
CREATE INDEX idx_reports_created_at ON reports(created_at);

-- Evidence table
CREATE TABLE evidence (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    evidence_type VARCHAR(50) NOT NULL,
    value TEXT NOT NULL,
    source VARCHAR(255) NOT NULL,
    submitted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    confidence DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_evidence_report_id ON evidence(report_id);
CREATE INDEX idx_evidence_submitted_by ON evidence(submitted_by);
CREATE INDEX idx_evidence_type ON evidence(evidence_type);

-- Suggestions table
CREATE TABLE suggestions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    reputation_id UUID NOT NULL REFERENCES reputations(id) ON DELETE CASCADE,
    suggested_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    suggested_status VARCHAR(50) NOT NULL,
    reason TEXT NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_suggestions_reputation_id ON suggestions(reputation_id);
CREATE INDEX idx_suggestions_suggested_by ON suggestions(suggested_by);
CREATE INDEX idx_suggestions_status ON suggestions(status);

-- Audit logs table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    details JSONB NOT NULL DEFAULT '{}',
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_resource_type ON audit_logs(resource_type);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

-- Email logs table (for SMTP gateway)
CREATE TABLE email_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    message_id VARCHAR(512),
    from_address VARCHAR(512) NOT NULL,
    to_addresses TEXT[] NOT NULL,
    subject VARCHAR(1024),
    risk_score INTEGER NOT NULL DEFAULT 0,
    decision VARCHAR(50) NOT NULL,
    auth_spf VARCHAR(50),
    auth_dkim VARCHAR(50),
    auth_dmarc VARCHAR(50),
    raw_headers TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_email_logs_from_address ON email_logs(from_address);
CREATE INDEX idx_email_logs_created_at ON email_logs(created_at);
CREATE INDEX idx_email_logs_decision ON email_logs(decision);

-- Insert default admin user (password: Admin123!)
-- Password hash generated with Argon2id
INSERT INTO users (email, password_hash, role, is_active, created_at, updated_at)
VALUES (
    'admin@m-eye.local',
    '$argon2id$v=19$m=19456,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXoxMjM0NTY$K7gNU3sR+gPE+FxyOlvS8dHqQhGzNvVJxLpTqUvWXYA',
    'administrator',
    true,
    NOW(),
    NOW()
)
ON CONFLICT (email) DO NOTHING;
