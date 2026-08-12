# Security Policy

## 🛡️ Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

We recommend always using the latest version for security patches and improvements.

## 🔒 Reporting a Vulnerability

We take the security of M'Eye seriously. If you discover a security vulnerability, please follow these guidelines:

### **DO NOT** create a public issue on GitHub

### How to Report

1. **Email**: Send your report to `security@m-eye.local` (configure in production)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
   - Your contact information for follow-up

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 5 business days
- **Resolution Timeline**: Depends on severity
  - Critical: 24-72 hours
  - High: 1 week
  - Medium: 2-4 weeks
  - Low: Next release cycle

### Disclosure Policy

- We will coordinate with you on public disclosure timing
- We appreciate responsible disclosure
- We will credit you (with your permission) in our security advisories

## 🔐 Security Best Practices for Users

### Production Deployment

1. **Change Default Credentials**
   ```bash
   # Change admin password immediately after installation
   ```

2. **Generate Secure Secrets**
   ```bash
   # JWT Secret
   openssl rand -base64 32
   
   # Database Password
   openssl rand -base64 32
   ```

3. **Use HTTPS/TLS**
   - Place M'Eye behind a reverse proxy (nginx, traefik)
   - Use valid SSL certificates
   - Enable HSTS

4. **Network Security**
   - Don't expose PostgreSQL directly to the internet
   - Use firewall rules to limit access
   - Consider using a VPN for admin access

5. **Regular Updates**
   - Keep Docker images updated
   - Monitor security advisories
   - Apply patches promptly

6. **Backup Strategy**
   - Regular database backups
   - Secure backup storage
   - Test restoration procedures

7. **Monitoring**
   - Enable audit logging
   - Monitor for suspicious activity
   - Set up alerts for anomalies

### Configuration Security

```env
# .env file - protect this file!

# Use strong, unique passwords
POSTGRES_PASSWORD=<strong-random-password>

# Generate secure JWT secret (min 32 bytes)
JWT_SECRET=<random-base64-string>

# Don't use default values in production
```

## 🏗️ Security Architecture

### Built-in Security Features

- **Password Hashing**: Argon2id (memory-hard, resistant to GPU attacks)
- **JWT Authentication**: Secure token-based auth with expiration
- **SQL Injection Prevention**: Parameterized queries via SQLx
- **Input Validation**: Strict validation on all user inputs
- **Rate Limiting**: Protection against brute force and DoS
- **RBAC**: Role-based access control
- **Audit Logging**: Complete trail of sensitive operations
- **Minimal Data Retention**: Only store what's necessary

### Security Headers (to be implemented in reverse proxy)

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Content-Security-Policy: default-src 'self'
```

## 🧪 Security Testing

### Automated Tests

```bash
# Run all tests including security-related
cargo test

# Run clippy for common issues
cargo clippy --all-targets --all-features -- -D warnings
```

### Manual Testing Checklist

- [ ] Authentication bypass attempts
- [ ] SQL injection in all input fields
- [ ] XSS in user-generated content
- [ ] CSRF protection
- [ ] Rate limiting effectiveness
- [ ] Authorization checks on all endpoints
- [ ] Session management
- [ ] Error handling (no information leakage)

## 📋 Known Security Considerations

### Current Limitations

1. **SMTP Gateway**: Currently runs on port 2525. In production, consider:
   - Running behind TLS terminator
   - Using authentication for relay
   - Implementing DKIM signing for outbound

2. **File Uploads** (Phase 2):
   - Will require strict type checking
   - Size limits
   - Sandboxed processing
   - No direct execution

3. **URL Analysis** (Phase 3):
   - Will use sandboxed environment
   - No direct server-side fetching without isolation
   - Rate limiting per domain

## 🔍 Security Audit Trail

All security-relevant events are logged:

- Authentication attempts (success/failure)
- Authorization failures
- Reputation changes
- Moderation decisions
- User role changes
- Configuration modifications

Logs can be viewed with:

```bash
docker compose logs -f backend
```

## 🎓 Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://rust-lang.github.io/secure-code/)
- [Docker Security Best Practices](https://docs.docker.com/engine/security/)
- [PostgreSQL Security](https://www.postgresql.org/docs/current/user-auth.html)

## 📞 Contact

For security-related inquiries:
- Email: `security@m-eye.local` (configure in production)
- GitHub Security Advisories: Enable in repository settings

---

**Remember**: Security is a shared responsibility. Thank you for helping keep M'Eye secure!
