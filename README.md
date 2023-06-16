# README

## Purpose

This PoC application is meant to evaluate claims a person may invoke on a given benefit.
Restrictions:
- The person must be identified
- The benefit claim might be re-evaluated at a later date


## API

Additionally, as  PoC, this application will offer API endpoints to permit:
- An person to be created with 1..n identities
- A benefit claim to be created
- A benefit claim's evaluation(s) to be fetched (including incomplete ones, if any)
- An endpoint to manually "progress" a specific evaluation
- A fully fleshed out evaluation when sufficient progress has been made (w/randomly generated content)


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

## Db Migrations

SQLx enables us to perform DB migrations at _run_time, even though it also helps us with a lot of magic at _compile_time.
Thus it should be perfectly safe to just upload a docker-image to any relevant runtime-platform whence a migration is desired.