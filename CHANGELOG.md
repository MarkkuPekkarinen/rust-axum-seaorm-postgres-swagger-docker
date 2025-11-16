# Changelog
All notable changes to this project will be documented in this file.

The format is based on **Keep a Changelog**  
and this project adheres to **Semantic Versioning (SemVer)**.

---

## [Unreleased]
### Added
- Placeholder for upcoming features.

### Changed
- Placeholder for upcoming changes.

### Fixed
- Placeholder for upcoming fixes.

---

## [v1.0.0] - 2025-01-01
### 🎉 Initial Release — Rust Axum + SeaORM + PostgreSQL + Swagger + Docker

#### Added
- Axum 0.8 backend server with clean layered structure  
  `routes → services → repositories → entities`
- SeaORM integration with PostgreSQL 17
- Automatic OpenAPI generation using `utoipa`
- Swagger UI exposed at `/swagger`
- Docker Compose environment (app + db + pgadmin)
- Hot-reload using cargo-watch (development only)
- PgAdmin 4 UI connected to PostgreSQL
- Full environment variables (`.env`) setup
- Production-ready Dockerfile (multi-stage)
- Project folder structure and documentation

---

## [v0.1.0] - 2024-12-01 (Optional)
### Added
- Project skeleton initialization.
- Basic Axum server & health check.
- Initial database schema & migrations.

---

## 🔗 Version Links
- **Unreleased:** https://github.com/asepindrak/rust-axum-seaorm-postgres-swagger-docker/compare/v1.0.0...HEAD  
- **v1.0.0:** https://github.com/asepindrak/rust-axum-seaorm-postgres-swagger-docker/releases/tag/v1.0.0  
