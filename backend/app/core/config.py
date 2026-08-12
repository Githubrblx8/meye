"""
M'Eye Backend Application Configuration
"""
from pydantic_settings import BaseSettings
from typing import Optional
import secrets


class Settings(BaseSettings):
    """Application settings"""
    
    # Application
    APP_NAME: str = "M'Eye"
    ENVIRONMENT: str = "development"
    SECRET_KEY: str = secrets.token_hex(32)
    JWT_EXPIRATION_MINUTES: int = 1440
    
    # Database
    DATABASE_URL: str = "postgresql://meye:change-me@db:5432/meye"
    
    # Redis
    REDIS_URL: str = "redis://redis:6379/0"
    
    # API
    API_PREFIX: str = "/api/v1"
    RATE_LIMIT_PER_MINUTE: int = 60
    
    # Security
    BCRYPT_ROUNDS: int = 12
    SESSION_TIMEOUT_HOURS: int = 24
    MAX_LOGIN_ATTEMPTS: int = 5
    
    # Logging
    LOG_LEVEL: str = "INFO"
    
    # Feature flags
    ENABLE_THREAT_INTEL: bool = False
    ENABLE_OPENSEARCH: bool = False
    
    # Admin account (initial setup)
    ADMIN_EMAIL: str = "admin@m-eye.local"
    ADMIN_PASSWORD: str = "ChangeMe123!"
    ADMIN_NAME: str = "Administrator"
    
    class Config:
        env_file = ".env"
        case_sensitive = True


settings = Settings()
