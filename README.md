<h1 align="center">
  <br>
      Recipya Rust
  <br>
</h1>

<h4 align="center">A clean, simple and powerful recipe manager your whole family will enjoy.</h4>

<p align="center">
    <a href="https://github.com/reaper47/recipya-rs/releases/latest" target="_blank" rel="noopener noreferrer">
        <img src="https://img.shields.io/github/v/release/reaper47/recipya-rs?style=flat&sort=semver">
    </a>
    <a href="https://github.com/reaper47/recipya-rs/actions/new" rel="noopener noreferrer">
        <img src="https://img.shields.io/github/actions/workflow/status/serde-rs/serde/ci.yml?branch=master">
    </a>
    <a href="https://github.com/reaper47/recipya-rs/issues" rel="noopener noreferrer">
        <img src="https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat" >
    </a>
</p>

<p align="center">
    <a href="https://recipes.musicavis.ca/guide/docs/" target="_blank" rel="noopener noreferrer">Website</a> •
    <a href="https://recipya.ca" target="_blank" rel="noopener noreferrer">Demo</a> •
    <a href="https://app.element.io/#/room/%23comfyui_space%3Amatrix.org" target="_blank" rel="noopener noreferrer">Matrix</a>
</p>

![Recipe page screenshot](.github/screenshot-recipes.webp)

Table of Contents
=================

* [Features](#features)
* [Installation](#installation)
  * [Docker](#docker)
      * [With compose](#with-compose)
      * [Without compose](#without-compose)
  * [Podman](#podman)
      * [Pod](#pod)
      * [Quadlets](#quadlets)
  * [Reverse proxy](#reverse-proxy)
* [Migrating from Recipya (Go version)](#migration-from-recipya-go-version)
* [Contributing](#contributing)
* [AI Disclaimer](#ai-disclaimer)
* [Sponsors](#sponsors)
* [Roadmap](#roadmap)
* [Inspiration](#inspiration)

## Features

- Manage your favorite recipes
- Import recipes from around the web
- Digitize paper recipes
- Organize your recipes into cookbooks
- Works seamlessly with [Nextcloud Cookbook](https://apps.nextcloud.com/apps/cookbook)
- Automatic conversion to your preferred measurement system (imperial/metric)
- Calculate nutritional information automatically
- Print any recipe in your collection
- Prevent your device from going to sleep while viewing a recipe
- Follows your system's theme (light/dark)
- Cross-compiled for Windows, Linux, and macOS

## Installation

The software can be installed using either Docker or Podman.

### Docker

#### With compose

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

#### Without compose

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

### Podman 

#### Pod

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

#### Quadlets

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

### Reverse proxy

If you use caddy to reverse proxy applications on your server, you may use the following [block](https://github.com/reaper47/recipya-rs/blob/main/deploy/caddyfile) to host Recipya.

## Migrating from Recipya (Go version)

1. Update your Docker image to latest `nightly` version.

2. Access the app

3. Export your data as JSON from the settings: `Settings → Data → Export data → JSON`

4. Import the exported archive into recipya-rs: `Add recipe → Import → Software`

## Contributing

Please consult the [contribution guide](./CONTRIBUTING.md) for details on how you can contribute to the project.

## AI Disclaimer

What is there to be proud of if your project is generated by AI? You didn't write your art. The machine did.

Recipya is a project where AI is used sparingly. And when it is used, it serves as a  rubber duck, a research tool, a debugging assistant and sometimes a guide on idiomatic Rust. I never ran an agent like Claude Code and Codex. All features were designed and planned by hand or thought of in the shower. The database was entirely designed on paper to see the relationships between tables. Some features, such as the [shopping list] one, [contact], and [SMS] are also entirely planned on paper before coding.

Functions that were entirely generated by AI have an `// AI usage:` comment. This usually happens when I don't know the domain of what I'm trying to solve enough. The Bananacat (the mascot) and the default recipe image were generated using ComfyUI. It would one day be great if a non-AI mascot could be created using [Inkscape]. 

I want Recipya to stay human and avoid becoming unmaintainable slop.

[shopping list]: https://github.com/reaper47/recipya-rs/issues/268
[contact]: https://github.com/reaper47/recipya-rs/issues/316
[SMS]: https://github.com/reaper47/recipya-rs/issues/310
[Inkscape]: https://inkscape.org/

## Sponsors

I am grateful for any support that motivates me to continue developing this project and to host it reliably.

You can sponsor me on
[GitHub Sponsors](https://github.com/sponsors/reaper47) or
[Buy Me a Coffee](https://www.buymeacoffee.com/macpoule).

Your support is greatly appreciated! A third of donations will be sent to the Armed Forces of Ukraine.

This project is supported by these kind people:
<img src="web/sponsors/out/sponsors.svg" style="width:100%;max-width:800px;"/>

## Roadmap

Please consult the [roadmap document](./ROADMAP.md) for details.

## Inspiration

This project was mainly coded to blasting the following albums:

- [1914 - Viribus Unitis](https://www.youtube.com/watch?v=IET5AyShGYc)
- [4am](https://www.youtube.com/watch?v=tBcPji_jRDc)
- [Abysmal Dawn - Phylogenesis](https://www.youtube.com/watch?v=xJMybqRMedk&pp=ygUMYWJ5c21hbCBkYXdu)
- [Archspire - Bleed the Future](https://www.youtube.com/watch?v=o8H9ahswldM)
- [Archspire - Too Fast to Die](https://www.youtube.com/watch?v=bKlxhjKfnMQ)
- [Astralborne - Eternity's End](https://www.youtube.com/watch?v=MilBEj5W9io)
- [Atavistia - Cosmic Warfare](https://www.youtube.com/watch?v=VjJ_zb4RF2E)
- [Beast In Black - Dark Connection](https://www.youtube.com/watch?v=7NyON-NzBr4)
- [Cattle Decapitation - Terrasite](https://www.youtube.com/watch?v=x6rEDMqM36I)
- [Desoration - NON](https://www.youtube.com/watch?v=QGagHu8EvyY)
- [Ensiferum - From Afar](https://www.youtube.com/watch?v=6r8OPu3SRSM)
- [Fires in the Distance - Echoes From Deep November](https://www.youtube.com/watch?v=dnOE5nm6rfo)
- [Kalmah - Swamplord](https://www.youtube.com/watch?v=FhMsOB88dfo&list=PLkROH3Eqs0T9b5E2WDDOS0JgYLf24_dNs)
- [Lofi Girl - lofi hip hop radio](https://www.youtube.com/watch?v=jfKfPfyJRdk)
- [Lofi Girl - synthwave radio](https://www.youtube.com/watch?v=4xDzrJKXOOY)
- [Mozart - Requiem Dm](https://www.youtube.com/watch?v=pBGVfwOLU1w0)
- [Necrophobic - In the Twilight Grey](https://www.youtube.com/watch?v=eDFD6YnMid8)
- [Pain - You Only Live Twice](https://www.youtube.com/watch?v=obgCEoLzLs4)
- [Sonata Arctica - Talviyö](https://www.youtube.com/watch?v=x6rEDMqM36I)
- [Wintersun - Wintersun](https://www.youtube.com/watch?v=W0M3HAMus7g&pp=ygUPd2ludGVyc3VuIGFsYnVt)
- [Wintersun - Time I & II](https://www.youtube.com/watch?v=dl3pkdAzHrw)
