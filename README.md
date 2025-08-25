This project is deployed on a single Hetzner CPX11 VPS (2 vCPU, 2 GB RAM).

The goal is to document progress alongside the source code, demonstrating a deliberately overengineered setup for a personal portfolio website. By running k3s on a minimal VPS, and separating the backend, frontend, and database into distinct services.

## Todo

- [ ] Initial and k3s setup
- [ ] Postgres DB storing blog posts
- [x] Auto TLS with Traefik & Let's Encrypt
- [ ] Axum backend
- [ ] React frontend
- [ ] At least 3 interesting blog posts

[Auto TLS setup](traefik/auto-tls.md)
