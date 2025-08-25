This project is deployed on a single Hetzner CPX11 VPS (2 vCPU, 2 GB RAM).

The goal is to document progress alongside the source code, demonstrating a deliberately overengineered setup for a personal portfolio website. By running k3s on a minimal VPS, and separating the backend, frontend, and database into distinct services.

## Todo

- [x] [Initial setup and k3s](initial-setup/prep-work.md)
- [ ] Postgres DB storing blog posts
- [x] [Auto TLS setup](traefik/auto-tls.md)
- [ ] Axum backend
- [ ] React frontend
- [ ] At least 3 interesting blog posts
