<div align="center">

# Portfolio

**Built with [stackup](https://github.com/MariosElef/stackup)**

**Topcoat** · $\color{#DEA584}{\textbf{axum}}$

$\color{#4169E1}{\textbf{Postgres}}$ · $\color{#FF4438}{\textbf{Redis}}$ · $\color{#47A248}{\textbf{MongoDB}}$ · $\color{#2496ED}{\textbf{Docker Compose}}$

</div>

---

- Frontend: Topcoat (Rust)
- Backend: axum (Rust)
- Services: Postgres, Redis, MongoDB, Local HTTPS (Caddy + mkcert)

## Commands

[`stackup`](https://github.com/MariosElef/stackup) is the command line for this project. Run it with no arguments for a menu of your projects, or from inside this folder:

```sh
stackup up              # build and start everything in Docker
stackup dev             # databases in Docker, backend and frontend with reload
stackup migrate         # apply new migrations (with sqlx or Alembic migrations)
stackup doctor          # can this machine run it? each failure names its fix
stackup build           # build the images
stackup deploy --host HOST   # build for linux, send the images over SSH, start them
stackup ci              # the checks CI runs
stackup fmt             # format the code
```

`deploy` needs SSH access to a server with Docker. It builds for `linux/amd64`, streams the images with `docker save | ssh docker load` (no registry), copies `docker-compose.yml` and `.env.prod` (as `.env`), and runs `docker compose up -d`. Try it first with `--dry-run`. Without stackup, `docker compose up --build` starts everything.

## Run

```sh
./scripts/certs.sh      # once: a trusted local certificate (mkcert)
docker compose up --build
```

## Where things are

- App: https://localhost:8443
- API: https://api.localhost:8443/health

## Configuration

Settings live in `.env`, which compose reads. `.env.example` is the committed template. For another environment run `stackup env --create dev` (it saves the current `.env` as `.env.local` first) and switch with `stackup env dev`. Everything except `.env.example` is gitignored.
