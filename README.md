# M'Eye — Email Security & Reputation Platform

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-green.svg)](https://www.docker.com/)

**M'Eye** est une plateforme open source de sécurité et de réputation des emails permettant aux particuliers, entreprises, administrateurs et chercheurs en cybersécurité de contrôler les emails entrants et sortants via une passerelle SMTP.

## 🎯 Objectif

Déterminer si une adresse email, un domaine ou une IP est :

- 🟢 **SAFE** — fiable
- ⚪ **UNKNOWN** — aucune information suffisante
- 🟠 **WATCH** — suspect / à surveiller
- 🔴 **BLOCKED** — bloqué
- 🟣 **COMPROMISED** — compromis ou associé à une compromission confirmée

## 🚀 Installation Rapide

### Prérequis

- Docker (version 20+)
- Docker Compose (version 2+)

### Démarrage en 1 commande

```bash
git clone https://github.com/ton-user/m-eye.git
cd m-eye
cp .env.example .env
docker compose up -d
```

L'interface web sera accessible sur : http://localhost:3000

L'API REST sera accessible sur : http://localhost:8000

### Script d'installation automatique

```bash
./install.sh
```

Ce script va :
- Vérifier que Docker est installé
- Créer le fichier `.env` avec des secrets sécurisés
- Démarrer les conteneurs
- Appliquer les migrations de base de données
- Créer le compte administrateur par défaut

## 🏗️ Architecture

```
                    ┌──────────────┐
                    │   M'Eye Web  │
                    │   + API      │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              │                         │
       ┌──────▼──────┐           ┌──────▼──────┐
       │ PostgreSQL  │           │    Redis    │
       │             │           │             │
       └─────────────┘           └──────┬──────┘
                                        │
                                 ┌──────▼──────┐
                                 │    Worker   │
                                 │ SMTP/Jobs   │
                                 └─────────────┘
```

### Composants

| Service | Port | Description |
|---------|------|-------------|
| Web UI + API | 3000 / 8000 | Interface web et API REST |
| PostgreSQL | 5432 | Base de données principale |
| Redis | 6379 | Cache et queues de tâches |
| Worker | - | Traitement asynchrone SMTP |

## 📡 Fonctionnalités

### Analyse SMTP

- ✅ SPF (Sender Policy Framework)
- ✅ DKIM (DomainKeys Identified Mail)
- ✅ DMARC (Domain-based Message Authentication)
- ✅ ARC (Authenticated Received Chain)
- ✅ Analyse des headers
- ✅ Vérification d'infrastructure (IP, ASN, rDNS)

### Moteur de Réputation

Chaque identité (email, domaine, IP, URL) reçoit :
- Un statut (SAFE/UNKNOWN/WATCH/BLOCKED/COMPROMISED)
- Un score de risque (0-100)
- Un niveau de confiance (0-1)
- Un historique complet

Le score est **explicable** : chaque facteur de risque est documenté.

### Signalements Communautaires

Les utilisateurs peuvent signaler :
- Spam
- Phishing
- Malware
- Credential harvesting
- Impersonation
- Fraud
- Compromised account
- Suspicious activity

### Système de Preuves

Joindre des preuves aux signalements :
- Headers d'emails
- URLs suspectes
- Hashes de fichiers
- Résultats SPF/DKIM/DMARC
- Informations DNS
- Screenshots

### RBAC (Role-Based Access Control)

| Rôle | Permissions |
|------|-------------|
| Member | Consulter, signaler, bloquer localement |
| Trusted Reporter | + poids accru dans la réputation |
| Security Researcher | + publier des analyses détaillées |
| Moderator | + valider/refuser les signalements |
| Administrator | Configuration globale, utilisateurs |

## 🌐 API

Documentation complète disponible sur : http://localhost:8000/docs

### Exemples d'utilisation

```bash
# Vérifier la réputation d'un email
curl http://localhost:8000/api/v1/reputation/email/phishing@example.com

# Vérifier un domaine
curl http://localhost:8000/api/v1/reputation/domain/example.com

# Vérifier une IP
curl http://localhost:8000/api/v1/reputation/ip/1.2.3.4

# Soumettre un signalement
curl -X POST http://localhost:8000/api/v1/reports \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "target": "phishing@example.com",
    "category": "phishing",
    "description": "Email de phishing détecté"
  }'
```

## 🔐 Sécurité

M'Eye applique des standards de sécurité élevés :

- Validation stricte des entrées
- RBAC complet
- Principe du moindre privilège
- Audit logs
- Rate limiting
- Protection contre les injections SQL
- Protection SSRF
- Chiffrement des secrets
- Conteneurs Docker non-root

## 📊 Statuts de Réputation

| Statut | Description | Score | Action |
|--------|-------------|-------|--------|
| 🟢 SAFE | Identité fiable | 0-20 | Autoriser |
| ⚪ UNKNOWN | Aucune information | 21-40 | Surveiller |
| 🟠 WATCH | Suspect | 41-60 | Examiner |
| 🔴 BLOCKED | Malveillant confirmé | 61-80 | Bloquer |
| 🟣 COMPROMISED | Compromis | 81-100 | Bloquer + alerter |

## 🧠 Principes Importants

1. `UNKNOWN` ne signifie jamais `MALICIOUS`
2. Une seule plainte ne suffit jamais à bloquer globalement
3. Toutes les classifications doivent être explicables
4. Les décisions sont traçables
5. Les preuves sont séparées des opinions
6. Les utilisateurs peuvent contester une classification
7. Minimisation des données personnelles
8. PostgreSQL est la source de vérité
9. Auto-hébergeable et open source

## 📚 Documentation

- [Architecture](docs/architecture.md)
- [Installation](docs/installation.md)
- [Configuration](docs/configuration.md)
- [API Reference](docs/api.md)
- [SMTP Gateway](docs/smtp.md)
- [Risk Engine](docs/risk-engine.md)
- [Sécurité](docs/security.md)
- [Contribution](CONTRIBUTING.md)

## 🛠️ Développement

### Structure du projet

```
m-eye/
├── docker-compose.yml
├── .env.example
├── install.sh
├── README.md
│
├── backend/
│   ├── Dockerfile
│   ├── requirements.txt
│   └── app/
│       ├── main.py
│       ├── api/
│       ├── models/
│       ├── services/
│       └── core/
│
├── frontend/
│   ├── Dockerfile
│   ├── package.json
│   └── src/
│
├── worker/
│   ├── Dockerfile
│   └── app/
│
├── migrations/
│
└── config/
```

### Lancer en mode développement

```bash
docker compose -f docker-compose.dev.yml up --build
```

### Tests

```bash
docker compose exec backend pytest
```

## 🤝 Contribuer

Voir [CONTRIBUTING.md](CONTRIBUTING.md) pour les guidelines de contribution.

## 📄 License

AGPL-3.0 — Voir [LICENSE](LICENSE) pour plus de détails.

## 🙏 Remerciements

Projet open source maintenu par la communauté.

---

**M'Eye** — Protecting your inbox, together.
