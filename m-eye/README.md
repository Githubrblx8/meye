# 🛡️ M'Eye - Plateforme Open Source de Sécurité Email

**M'Eye** est une plateforme open source de sécurité et de réputation des emails permettant aux particuliers, entreprises et chercheurs en cybersécurité de contrôler les emails entrants et sortants.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.75-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-Ready-blue.svg)](https://www.docker.com)

## 🎯 Fonctionnalités

- **📧 SMTP Gateway** - Analyse des emails entrants/sortants
- **🔐 Authentication Checks** - SPF, DKIM, DMARC, ARC
- **🧠 Reputation Engine** - Scoring basé sur des standards industriels
- **⚠️ Risk Engine** - Décision ALLOW/WATCH/BLOCK/QUARANTINE
- **👥 Community Reports** - Signalements communautaires avec preuves
- **🔎 Threat Intelligence** - Intégration de sources externes (Phase 3)
- **📊 Dashboard** - Interface web moderne (Phase 2)

## 🚀 Installation Rapide (Windows 11)

### Prérequis

1. **Docker Desktop** pour Windows avec WSL2 activé
   - Télécharger: https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe
   - Activer WSL2 lors de l'installation

2. **Git for Windows**
   - Télécharger: https://gitforwindows.org/

### Étapes d'Installation

```powershell
# 1. Cloner le projet
git clone https://github.com/votre-org/m-eye.git
cd m-eye

# 2. Copier le fichier d'environnement
cp .env.example .env

# 3. Éditer .env et changer JWT_SECRET (IMPORTANT!)
# Utilisez un générateur de mot de passe sécurisé

# 4. Lancer l'application
docker compose up -d

# 5. Vérifier les logs
docker compose logs -f app
```

### Accès à l'Application

- **🌐 API**: http://localhost:3000
- **📚 Documentation API**: http://localhost:3000/docs
- **📧 SMTP Gateway**: localhost:2525

### Compte Administrateur par Défaut

- **Email**: `admin@m-eye.local`
- **Mot de passe**: `Admin123!`

> ⚠️ **CHANGEZ LE MOT DE PASSE IMMÉDIATEMENT** après la première connexion !

## 🏗️ Architecture

```
                    ┌──────────────┐
                    │  M'Eye App   │
                    │  (Rust/Axum) │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              │                         │
       ┌──────▼──────┐           ┌──────▼──────┐
       │ PostgreSQL  │           │    Redis    │
       │   (Data)    │           │  (Cache)    │
       └─────────────┘           └─────────────┘
```

### Services Docker

| Service | Port | Description |
|---------|------|-------------|
| `app` | 3000, 2525 | API + SMTP Gateway |
| `db` | 5432 (interne) | PostgreSQL 15 |
| `redis` | 6379 (interne) | Redis 7 |

## 📖 Utilisation

### Vérifier une Réputation

```bash
curl http://localhost:3000/api/v1/reputation/email/phishing@example.com
```

### Soumettre un Signalement

```bash
curl -X POST http://localhost:3000/api/v1/reports \
  -H "Content-Type: application/json" \
  -d '{
    "target_type": "email",
    "target_value": "spam@example.com",
    "category": "spam",
    "description": "Campagne de spam détectée"
  }'
```

### Configurer Votre Serveur Mail

Pour utiliser M'Eye comme gateway SMTP, configurez votre serveur mail (Postfix, Exchange, etc.) pour relayer vers `m-eye:2525`.

**Exemple Postfix** (`/etc/postfix/main.cf`):
```bash
relayhost = [m-eye]:2525
```

## 🔧 Configuration

Éditez `.env` pour personnaliser :

```bash
# Database
POSTGRES_PASSWORD=VotreMotDePasseSecurise!

# Security (CRITIQUE: Changez ceci!)
JWT_SECRET=votre_secret_tres_long_et_aleatoire_ici

# Risk Thresholds
RISK_THRESHOLD_BLOCK=60
```

## 🧪 Tests

```bash
# Builder et tester
docker compose build
docker compose run --rm app cargo test

# Tests spécifiques
docker compose run --rm app cargo test -p meye-risk
```

## 📚 Documentation Complète

- [Architecture Détaillée](docs/architecture.md)
- [Guide d'Installation](docs/installation.md)
- [API Reference](http://localhost:3000/docs)
- [Contribuer](CONTRIBUTING.md)
- [Sécurité](SECURITY.md)

## 🛣️ Roadmap

### Phase 1 (Actuelle) ✅
- [x] Core API Rust
- [x] Database Schema
- [x] Reputation Engine
- [x] Risk Engine
- [x] SMTP Gateway (basique)
- [x] Authentification JWT

### Phase 2
- [ ] Frontend React/TypeScript
- [ ] Dashboard complet
- [ ] Gestion des preuves
- [ ] Modération
- [ ] RBAC complet

### Phase 3
- [ ] OpenSearch integration
- [ ] Threat Intelligence providers
- [ ] Advanced analytics
- [ ] Researcher dashboard

### Phase 4
- [ ] Production hardening
- [ ] Monitoring avancé
- [ ] Workers distribués
- [ ] Fédération communautaire

## 🔐 Sécurité

Ce projet suit les meilleures pratiques :
- ✅ Mots de passe hashés avec Argon2id
- ✅ JWT pour l'authentification
- ✅ Conteneurs non-root
- ✅ Validation stricte des entrées
- ✅ Audit logs complets
- ✅ Rate limiting

**Reportez les vulnérabilités via** [SECURITY.md](SECURITY.md)

## 🤝 Contribuer

Les contributions sont les bienvenues ! Veuillez lire [CONTRIBUTING.md](CONTRIBUTING.md) avant de soumettre une PR.

## 📄 Licence

Ce projet est sous licence **AGPL-3.0**. Voir [LICENSE](LICENSE) pour plus de détails.

## 🙏 Remerciements

- La communauté Rust pour ses outils exceptionnels
- Les projets open source de sécurité email qui ont inspiré M'Eye
- Tous les contributeurs

---

**M'Eye** - *See everything, trust nothing.* 👁️
