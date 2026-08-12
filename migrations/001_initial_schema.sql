-- M'Eye Initial Database Schema

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    hashed_password VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'member' NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    last_login TIMESTAMP WITH TIME ZONE
);

-- Create index on email
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Reputations table
CREATE TYPE identity_type AS ENUM ('email', 'domain', 'ip', 'url');
CREATE TYPE reputation_status AS ENUM ('SAFE', 'UNKNOWN', 'WATCH', 'BLOCKED', 'COMPROMISED');

CREATE TABLE IF NOT EXISTS reputations (
    id SERIAL PRIMARY KEY,
    identity VARCHAR(512) UNIQUE NOT NULL,
    identity_type identity_type NOT NULL,
    status reputation_status DEFAULT 'UNKNOWN' NOT NULL,
    risk_score INTEGER DEFAULT 0 NOT NULL,
    confidence FLOAT DEFAULT 0.0 NOT NULL,
    reports_count INTEGER DEFAULT 0 NOT NULL,
    confirmed_incidents INTEGER DEFAULT 0 NOT NULL,
    first_seen TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP WITH TIME ZONE,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    risk_factors TEXT
);

CREATE INDEX IF NOT EXISTS idx_reputations_identity ON reputations(identity);
CREATE INDEX IF NOT EXISTS idx_reputations_type ON reputations(identity_type);
CREATE INDEX IF NOT EXISTS idx_reputations_status ON reputations(status);

-- Reputation history table
CREATE TABLE IF NOT EXISTS reputation_history (
    id SERIAL PRIMARY KEY,
    reputation_id INTEGER NOT NULL REFERENCES reputations(id) ON DELETE CASCADE,
    previous_status reputation_status NOT NULL,
    previous_risk_score INTEGER NOT NULL,
    previous_confidence FLOAT NOT NULL,
    new_status reputation_status NOT NULL,
    new_risk_score INTEGER NOT NULL,
    new_confidence FLOAT NOT NULL,
    reason TEXT,
    changed_by VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reputation_history_reputation_id ON reputation_history(reputation_id);

-- Reports table
CREATE TYPE report_category AS ENUM (
    'spam', 'phishing', 'malware', 'credential_harvesting',
    'impersonation', 'fraud', 'compromised_account', 'suspicious', 'other'
);

CREATE TYPE report_status AS ENUM ('pending', 'under_review', 'confirmed', 'rejected', 'duplicate');

CREATE TABLE IF NOT EXISTS reports (
    id SERIAL PRIMARY KEY,
    target VARCHAR(512) NOT NULL,
    reporter_id INTEGER NOT NULL REFERENCES users(id),
    category report_category NOT NULL,
    description TEXT NOT NULL,
    status report_status DEFAULT 'pending' NOT NULL,
    reviewed_by INTEGER REFERENCES users(id),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    review_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_reports_target ON reports(target);
CREATE INDEX IF NOT EXISTS idx_reports_reporter ON reports(reporter_id);
CREATE INDEX IF NOT EXISTS idx_reports_status ON reports(status);

-- Evidence table
CREATE TYPE evidence_type AS ENUM (
    'email_headers', 'url', 'domain', 'ip', 'file_hash', 'screenshot',
    'spf_result', 'dkim_result', 'dmarc_result', 'dns_information',
    'sandbox_result', 'threat_intel_reference', 'analyst_report', 'other'
);

CREATE TABLE IF NOT EXISTS evidence (
    id SERIAL PRIMARY KEY,
    report_id INTEGER NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    submitted_by INTEGER NOT NULL REFERENCES users(id),
    evidence_type evidence_type NOT NULL,
    value TEXT NOT NULL,
    source VARCHAR(255),
    confidence FLOAT DEFAULT 0.5 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_evidence_report_id ON evidence(report_id);
CREATE INDEX IF NOT EXISTS idx_evidence_submitted_by ON evidence(submitted_by);

-- Suggestions table
CREATE TYPE suggested_status AS ENUM ('SAFE', 'WATCH', 'BLOCKED', 'COMPROMISED');

CREATE TABLE IF NOT EXISTS suggestions (
    id SERIAL PRIMARY KEY,
    target VARCHAR(512) NOT NULL,
    submitted_by INTEGER NOT NULL REFERENCES users(id),
    suggested_status suggested_status NOT NULL,
    reason VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) DEFAULT 'pending' NOT NULL,
    reviewed_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    reviewed_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_suggestions_target ON suggestions(target);
CREATE INDEX IF NOT EXISTS idx_suggestions_status ON suggestions(status);

-- Insert default admin user
-- Password: ChangeMe123! (hashed with bcrypt)
INSERT INTO users (email, name, hashed_password, role, is_active)
VALUES (
    'admin@m-eye.local',
    'Administrator',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzS3MebAJu',
    'admin',
    TRUE
) ON CONFLICT (email) DO NOTHING;
