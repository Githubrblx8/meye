# Authentification JWT Implémentée

L'authentification JWT est maintenant complètement implémentée dans M'Eye.

## 📋 Composants

### 1. Module JWT (`src/core/jwt.rs`)

- **JwtClaims** : Structure des claims JWT (sub, email, role, iss, iat, exp)
- **JwtManager** : Gestionnaire de tokens JWT
  - `generate_token()` : Génère un token signé
  - `validate_token()` : Valide et décode un token
  - `extract_user_id_unsafe()` : Extrait l'ID sans validation (pour logs)
- **JwtError** : Types d'erreurs JWT

### 2. Endpoints d'Authentification (`src/api/auth.rs`)

#### POST `/api/v1/auth/login`
```json
{
  "email": "user@example.com",
  "password": "secure_password_123"
}
```

Réponse :
```json
{
  "access_token": "eyJhbGciOiJIUzIyNiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "uuid-here",
    "email": "user@example.com",
    "role": "member"
  }
}
```

#### POST `/api/v1/auth/register`
```json
{
  "email": "newuser@example.com",
  "password": "very_secure_password_minimum_12_chars"
}
```

Réponse :
```json
{
  "id": "uuid-here",
  "email": "newuser@example.com",
  "message": "User registered successfully. Please login."
}
```

#### GET `/api/v1/auth/me`
Nécessite un token Bearer valide dans le header `Authorization`.

Réponse :
```json
{
  "id": "uuid-here",
  "email": "user@example.com",
  "role": "member",
  "created_at": "2024-01-01T00:00:00Z"
}
```

### 3. Middleware d'Authentification (`src/api/middleware.rs`)

- **extract_bearer_token()** : Extrait le token du header Authorization
- **require_auth()** : Middleware pour routes nécessitant authentification
- **require_role()** : Middleware pour routes nécessitant un rôle spécifique
- **has_required_role()** : Vérifie la hiérarchie des rôles

Hiérarchie des rôles :
```
Member (0) < TrustedReporter (1) < SecurityResearcher (2) < Moderator (3) < Administrator (4)
```

### 4. Configuration (`src/main.rs`)

Variables d'environnement :
- `JWT_SECRET` : Clé secrète pour signer les tokens (min 32 caractères recommandé)
- `JWT_ISSUER` : Émetteur du token (défaut: "m-eye-api")
- `JWT_LIFETIME_SECS` : Durée de vie du token en secondes (défaut: 3600)

## 🔐 Sécurité

### Hashage des mots de passe
- Algorithme : **Argon2id** (meilleure pratique actuelle)
- Salt généré aléatoirement pour chaque utilisateur
- Longueur minimale : 12 caractères

### Validation des tokens
- Signature vérifiée avec HMAC-SHA256
- Expiration vérifiée automatiquement
- Issuer vérifié
- Claims extraits et validés

### Protection contre les abus
- Rate limiting (à implémenter au niveau middleware)
- Logs des tentatives échouées
- Timing constant pour les vérifications de mot de passe

## 🧪 Tests

Le module JWT inclut des tests unitaires :

```bash
cd backend
cargo test core::jwt
```

Tests inclus :
- Génération et validation de token
- Token expiré
- Signature invalide

## 📝 Utilisation dans les handlers

### Récupérer les claims dans un handler

```rust
use axum::extract::Extension;
use crate::core::jwt::JwtClaims;

async fn protected_handler(
    Extension(claims): Extension<JwtClaims>,
) -> Result<Json<Response>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Utiliser user_id et claims.role
    Ok(Json(response))
}
```

### Protéger une route avec middleware

```rust
use crate::api::middleware::require_auth;

Router::new()
    .route(
        "/api/v1/protected",
        get(protected_handler).layer(middleware::from_fn(require_auth))
    )
```

### Route nécessitant un rôle spécifique

```rust
use crate::api::middleware::require_role;
use crate::models::UserRole;

Router::new()
    .route(
        "/api/v1/admin",
        get(admin_handler).layer(middleware::from_fn_with_state(
            UserRole::Administrator,
            require_role
        ))
    )
```

## 🚀 Démarrage

1. Copier `.env.example` vers `.env`
2. Modifier `JWT_SECRET` avec une valeur forte :
   ```bash
   openssl rand -base64 32
   ```
3. Démarrer avec `docker compose up -d`
4. Tester l'API :
   ```bash
   # Créer un compte
   curl -X POST http://localhost:8000/api/v1/auth/register \
     -H "Content-Type: application/json" \
     -d '{"email":"test@example.com","password":"TestPassword123!"}'
   
   # Se connecter
   curl -X POST http://localhost:8000/api/v1/auth/login \
     -H "Content-Type: application/json" \
     -d '{"email":"test@example.com","password":"TestPassword123!"}'
   
   # Accéder à une route protégée
   curl http://localhost:8000/api/v1/auth/me \
     -H "Authorization: Bearer YOUR_TOKEN_HERE"
   ```

## 📖 Références

- [JWT Standard](https://jwt.io/)
- [Argon2 RFC](https://datatracker.ietf.org/doc/html/rfc9106)
- [Axum Middleware](https://docs.rs/axum/latest/axum/middleware/index.html)
