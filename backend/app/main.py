"""
M'Eye Backend Application
"""
from fastapi import FastAPI, Depends, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from slowapi import SlowAPISlow
from slowapi.util import get_remote_address
from .core.config import settings
from .core.database import engine, Base, get_db
from .api import auth, reputation, reports, users, health

# Create tables
Base.metadata.create_all(bind=engine)

# Initialize FastAPI app
app = FastAPI(
    title=settings.APP_NAME,
    description="Email Security & Reputation Platform",
    version="1.0.0",
    openapi_url=f"{settings.API_PREFIX}/openapi.json",
    docs_url=f"{settings.API_PREFIX}/docs",
    redoc_url=f"{settings.API_PREFIX}/redoc"
)

# Rate limiter
limiter = SlowAPISlow(key_func=get_remote_address)
app.state.limiter = limiter

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(auth.router, prefix=f"{settings.API_PREFIX}/auth", tags=["Authentication"])
app.include_router(reputation.router, prefix=f"{settings.API_PREFIX}/reputation", tags=["Reputation"])
app.include_router(reports.router, prefix=f"{settings.API_PREFIX}/reports", tags=["Reports"])
app.include_router(users.router, prefix=f"{settings.API_PREFIX}/users", tags=["Users"])
app.include_router(health.router, prefix=f"{settings.API_PREFIX}", tags=["Health"])


@app.get("/")
async def root():
    """Root endpoint"""
    return {
        "name": settings.APP_NAME,
        "version": "1.0.0",
        "description": "Email Security & Reputation Platform",
        "docs": f"{settings.API_PREFIX}/docs"
    }


# Exception handler for rate limiting
@app.exception_handler(429)
async def rate_limit_handler(request, exc):
    return {
        "detail": "Rate limit exceeded. Please try again later.",
        "status_code": 429
    }
