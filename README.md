# README

## Set-up of dev-env
Ensure you've got:
1. Nix installed (nix flake compatible)
1. Docker daemon running on your system
1. `docker-compose` (if docker is installed on Windows)
1. `sqlx` cli tool

Then run:
1. `docker-compose up -d`
1. `sqlx database reset -y --source database-migrations/`

Now you can run:
`cargo --frozen watch -cqw src/ -x 'run -q'` while developing your Rust/SQL code.

## Working with WSL and docker compose

Use `docker.exe` and `docker-compose.exe` from inside wsl, while making sure docker desktop in windows is running.
Also, in docker desktop: settings -> resources -> wsl-integration: make sure docker-wsl integration is enabled.
