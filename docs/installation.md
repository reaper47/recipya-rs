# Installation

The software can be installed using either Docker or Podman.

## Table of Contents

* [Docker](#docker)
  * [With compose](#with-compose)
  * [Without compose](#without-compose)
* [Podman](#podman)
  * [Pod](#pod)
  * [Quadlets](#quadlets)
* [Reverse proxy](#reverse-proxy)

## Docker

### With compose

1. Copy the [docker-compose.yml](./deploy/docker-compose.yml) file to where you want:
```bash
mkdir recipya-rs-compose && cd recipya-rs-compose
wget https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/docker-compose.yml 
```

2. Fetch and review the `.env` configuration file:
```bash
wget -O .env https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/.env.example 
```

3. Start the services
```bash
docker compose up -d
```

### Without compose

1. Create the network and volumes:
```bash
docker network create recipya-net
docker volume create recipya-rs-db
docker volume create recipya-data
```

2. Start the database container:
```bash
docker run -d \
  --name recipya-rs-db \
  --network recipya-net \
  --network-alias recipya-db \
  --restart unless-stopped \
  -e POSTGRES_DB=recipya \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -v recipya-rs-db:/var/lib/postgresql \
  -p 5432:5432 \
  --health-cmd "pg_isready -U postgres" \
  --health-interval 3s \
  --health-timeout 5s \
  --health-retries 10 \
  reaper99/recipya-rs-db:nightly
```

3. Fetch and review the `.env` configuration file:
```bash
wget -O .env https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/.env.example 
```

4. Start the web server container:
```bash
docker run -d \
  --name recipya \
  --network recipya-net \
  --restart unless-stopped \
  --env-file ./.env \
  -v recipya-data:/home/recipya/.local/share/Recipya \
  -p 8078:8078 \
  reaper99/recipya-rs:nightly
```

## Podman 

### Pod

1. Create a pod named `recipya-rs` with published ports 8078 (Recipya web server) and 5432 (PostgreSQL):
```bash
podman pod create --name recipya-rs -p 8078:8078 -p 5432:5432
```

2. Create the volumes, one for the database and another for the Recipya web server:
```bash
podman volume create recipya-rs-db
podman volume create recipya-rs-data
```

3. Start the database container in the recipya-rs pod:
```bash
podman run -d --pod recipya-rs --name recipya-rs-db --restart unless-stopped -e POSTGRES_DB=recipya -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -v recipya-rs-db:/var/lib/postgresql reaper99/recipya-rs-db:latest
```

4. Fetch and review the `.env` configuration file:
```bash
wget -O .env https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/.env.example 
```

5. Start the web server's container in the recipya-rs pod:
```bash
podman run -d --pod recipya-rs --name recipya-rs-app --restart unless-stopped \
    --env-file ./.env \
    -v recipya-rs-data:/home/recipya/.local/share/Recipya \
    --health-cmd "curl -f http://localhost:8078/health/ready || exit 1" \
    --health-interval 30s \
    --health-timeout 10s \
    --health-start-period 5s \
    --health-retries 3 \
    reaper99/recipya-rs:latest
```

### Quadlets

This is how the [demo](https://recipya.ca) is hosted.

1. Navigate to the Podman storage directory:
```bash
cd ~/config/containers/systemd
```

2. Download the files:
```bash 
wget https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-app.container \
https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-data.volume \
https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-db-data.volume \
https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-db.container \
https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-db.env \
https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/podman/recipya-rs-net.network

wget https://raw.githubusercontent.com/reaper47/recipya-rs/refs/heads/main/deploy/.env.example -O recipya-rs.env
```

3. The recipya-rs-db is a PostgreSQL database exposed on port 5432. If your host already has something listening on port 5432, please update the host port mapping to avoid conflicts.:
```bash
vi recipya-rs-db.container

# Modify line 12
PublishPort=[NEW_PORT]:5432
```

4. The default PostgreSQL credentials are `postgres:postgres`. If you wish to use other credentials:
```bash
vi recipya-rs-db.env

# Modify lines 2 and 3
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
```

5. Review the environment variables:
```bash
vi recipya-rs.env
```

6. Restart the daemon:
```bash
systemctl --user daemon-reload
```

7. Check the status:
```bash
systemctl --user status recipya-rs-app.service
systemctl --user status recipya-rs-db.service
```

8. Start the services:
```bash
systemctl --user start recipya-rs-app.service
systemctl --user start recipya-rs-db.service
```

9. You can check the status once more to ensure the service is running. You should see this entry in recipya-rs-app
```text
2026-06-03T18:00:09.080273Z  INFO Serving at http://0.0.0.0:8078
```

10. To update the software:
```bash
podman pull docker.io/reaper99/recipya-rs:nightly
systemctl --user restart  recipya-rs-app.service
```

## Reverse proxy

If you use caddy to reverse proxy applications on your server, you may use the following [block](https://github.com/reaper47/recipya-rs/blob/main/deploy/caddyfile) to host Recipya.
