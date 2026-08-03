<div align="center">

# Recipya Rust &emsp;

**A clean, simple and powerful recipe manager your whole family will enjoy.**

[![Demo][demo-shield]][demo-url]
[![Documentation][docs-shield]][docs-url]
[![Matrix][matrix-shield]][matrix-url]
<br>
[![][github-release-shield]][github-release-link]
[![build-status-shield]][build-status-url]
[![contributions-shield]][contributions-url]

[build-status-shield]: https://img.shields.io/github/actions/workflow/status/serde-rs/serde/ci.yml?branch=master

[build-status-url]: https://github.com/reaper47/recipya-rs/actions/new

[contributions-shield]: https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat

[contributions-url]: https://github.com/reaper47/recipya-rs/issues

[demo-shield]: https://img.shields.io/badge/demo-996300?style=flat

[demo-url]: https://recipes.musicavis.ca

[docs-shield]: https://img.shields.io/badge/documentation-5d782e?style=flat

[docs-url]: https://recipes.musicavis.ca/guide/docs/

[matrix-shield]: https://img.shields.io/badge/Matrix-000000?style=flat&logo=matrix&logoColor=white

[matrix-url]: https://app.element.io/#/room/%23comfyui_space%3Amatrix.org

[github-release-shield]: https://img.shields.io/github/v/release/reaper47/recipya-rs?style=flat&sort=semver

[github-release-link]: https://github.com/reaper47/recipya-rs/releases

![Recipe page screenshot](.github/screenshot-recipes.webp)

</div>

## Introduction

A clean, simple and powerful recipe manager web application for unforgettable family recipes, empowering you to curate
and share your favorite recipes. It is focused on simplicity for the whole family to enjoy.

## :warning: Important Note 

The [Recipya](https://github.com/reaper47/recipya) project is currently being rewritten in Rust as
announced [here](https://github.com/reaper47/recipya/discussions/422). The Rust project is not ready for production nor
to selfhost.
The user interface is better than the the original.

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

## Getting Started

The instructions will be written later. For now, you can use the Docker or Podman solution under the deploy folder.

## Development

### Linux

Follow these instructions to set up your development environment on Linux.

#### Tooling

1. Install [Rust](https://www.rust-lang.org/)
1. Install [Nodejs](https://nodejs.org/en/download/package-manager)
1. Install [PostgreSQL](https://www.postgresql.org/download/) 
1. Clone and setup the [pg_cron extension](https://github.com/citusdata/pg_cron)
1. Modify `/var/lib/pgsql/data/postgresql.conf`:
    - `shared_preload_libraries = 'pg_cron'` <- Uncomment this line
    - `cron.database_name = 'recipya'` <- Add line
1. Install [Diesel](https://diesel.rs/): `cargo install diesel_cli --no-default-features --features postgres`
1. Install [cargo-metask](https://crates.io/crates/cargo-metask): `cargo install cargo-metask`
1. Open a terminal and connect to postgres: `psql -U postgres`
1. Create the Recipya database: `CREATE DATABASE recipya;`
1. Create the Recipya test database: `CREATE DATABASE recipya_test;`

#### Running Recipya

1. Clone the repository: `git clone https://github.com/reaper47/recipya-rs.git`
1. Open the repository in your IDE
1. Install the NPM dependencies: `cargo task web-install-deps`
1. Copy the `deploy/.env.example` file to the root of the repository and rename it to `.env`
1. Update the `DATABASE_URL` variable to point to your local Postgres database
1. Run the SQL migrations: `diesel migration run`
1. Run `cargo run server` to start the web server

### Windows

Follow these instructions to set up your development environment on Windows if you do not use WSL nor Docker.

#### Tooling

1. Install Git: `winget install --id Git.Git -e --source winget`
1. Install Rust using [rustup-init](https://rust-lang.org/tools/install/)
1. Install [Postgresql 17.XX](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads)
1. Add the PostgreSQL lib to your user LIB: `C:\Program Files\PostgreSQL\17\lib`
1. Add the PostgreSQL bin to your user PATH: `C:\Program Files\PostgreSQL\17\bin`
1. Install Visual Studio "Desktop development with C++" using the Visual Studio Installer
1. Install NASM: `winget install NASM.NASM`
1. Install LLVM: `winget install LLVM.LLVM`
1. Install the [PostgreSQL pg_cron extension](https://github.com/hakanrw/pg_cron_windows#building-on-windows)
    - Download and unzip the [latest release zip](https://github.com/hakanrw/pg_cron_windows/releases)
    - Install [GNU Make](https://gnuwin32.sourceforge.net/packages/make.htm)
    - Add the GNU Make bin directory to your user system PATH: `C:\Program Files (x86)\GnuWin32\bin`
    - Run the `Git Bash` app as admin
    - Navigate to the unzipped release directory: `cd /c/Users/<username>/Downloads/pg_cron_windows_msvc_x64/pg_cron/`
    - Run `make install`
1. Modify `C:\Program Files\PostgreSQL\17\data\postgresql.conf`:
    - `shared_preload_libraries = 'pg_cron'` <- Uncomment this line
    - `cron.database_name = 'recipya'` <- Add line
1. Install [cargo-metask](https://crates.io/crates/cargo-metask): `cargo install cargo-metask`
1. Install [Diesel](https://diesel.rs): `cargo install diesel_cli --no-default-features --features postgres`
1. Open a command prompt and connect to postgres: `psql -U postgres`
1. Create the Recipya database: `CREATE DATABASE recipya;`
1. Create the Recipya test database: `CREATE DATABASE recipya_test;`

#### Running Recipya

1. Clone the repository: `git clone https://github.com/reaper47/recipya-rs.git`
1. Open the repository in your IDE
1. Install the NPM dependencies: `cargo task web-install-deps`
1. Copy the `deploy\.env.example` file to the root of the repository and rename it to `.env`
1. Update the `DATABASE_URL` variable to point to your local Postgres database
1. Run the SQL migrations: `diesel migration run`
1. Run `cargo run server` to start the web server

### Development Container

You may use the devcontainer to help develop Recipya. The `DATABASE_URL` environment variable in your
`.env` file would be `DATABASE_URL = "postgres://postgres:postgres@localhost:5432"`.

### Useful Commands

#### Frontend

Run `cargo task web-build` to build the frontend.

#### Database

Run `diesel migration run` to run all pending migrations.

Run `diesel migration generate {migration_name}` to create a new SQL migration under [crates/repository/src/migrations](https://github.com/reaper47/recipya-rs/tree/main/crates/repository/src/migrations) when changes to the database need to be done.

Run `diesel migration redo` to rerun the latest migration.

Run `diesel migration redo --all` to rerun all migrations.

Run `./.devcontainer/clean_test_dbs.sh` to clean up any test databases.

## Contributing

Contributions are always welcome! Please open an issue, start
a [discussion](https://github.com/reaper47/recipya/discussions), open a pull request or send an email
at macpoule@gmail.com. The same applies if you have any feedback or need support.

You can also join our development and support channel on
the [Matrix space: #recipya:matrix.org](https://app.element.io/#/room/#recipya:matrix.org).
Matrix is similar to Discord but is open source.

## Sponsors

I am grateful for any support that motivates me to continue developing this project and to host it reliably.

You can sponsor me on
[GitHub Sponsors](https://github.com/sponsors/reaper47) or
[Buy Me a Coffee](https://www.buymeacoffee.com/macpoule).

Your support is greatly appreciated! A third of donations will be sent to the Armed Forces of Ukraine 🇺🇦

This project is supported by these kind people:
<img src="web/sponsors/out/sponsors.svg" style="width:100%;max-width:800px;"/>

# Inspiration

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
