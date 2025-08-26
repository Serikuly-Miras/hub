This project is deployed on a single Hetzner CPX11 VPS (2 vCPU, 2 GB RAM).

The goal is to document progress alongside the source code, demonstrating a **deliberately overengineered** setup for a personal portfolio website. By running k3s on a VPS, and separating the backend, frontend, and database into distinct services.

## Progress

- [x] [Initial setup and k3s](initial-setup/prep-work.md)
- [x] [Auto TLS setup](traefik/auto-tls.md)
- [ ] Axum backend
  - Setup
    - [ ] Migrations
    - [ ] Dockerfile
  - Features
    - [ ] API endpoints
    - [ ] Caching
  - Quality
    - [ ] Docs
    - [ ] Tests
    - [ ] Benchmarks
- [ ] React frontend
  - Setup
    - [ ] Dockerfile
  - Features
    - [ ] Routing
    - [ ] API integration
    - [ ] State management
  - Quality
    - [ ] Docs
    - [ ] Tests
    - [ ] Linting/Formatting
- [ ] CI/CD
- [ ] At least 3 interesting blog posts
