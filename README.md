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

* [Important Note](#warning-important-note)
* [Features](#features)
* [Getting Started](#getting-started)
* [Development](#development)
  * [Linux](#linux)
      * [Tooling](#tooling)
      * [Running Recipya](#running-recipya)
  * [Windows](#windows)
      * [Tooling](#tooling-1)
      * [Running Recipya](#running-recipya-1)
  * [Development Container](#development-container)
  * [Useful Commands](#useful-commands)
      * [Frontend](#frontend)
      * [Database](#database)
* [Contributing](#contributing)
  * [How to support/verify a website](#how-to-supportverify-a-website)
      * [Example](#example)
* [Sponsors](#sponsors)
* [Inspiration](#inspiration)

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

### How to support/verify a website

1. Open the respective [registry file](https://github.com/reaper47/recipya-rs/tree/main/crates/recipya-scraper/data)
2. Add the website's required metadata (name, domain, variant, url, test urls), and if applicable, the optional cuisine
3. Save the file. A Rust file with the added website will be automatically generated. Run `cargo build` if it does not generate.
4. Open the website's respective [test file](https://github.com/reaper47/recipya-rs/tree/main/crates/recipya-scraper/src/tests)
5. Add the test skeleton for the website:
```rust
#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_{website name}_ok() -> Result<()> {
    let got = scrape(Website::{website name}, 0)?;

    pretty_assertions::assert_eq!(got, Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        url: vec!["the website url from the registry".into()],
        ..Default::default()
    });
    Ok(())
}
```
6. Run the test.
   1. If you don't receive a `DomainNotImplemented` error, populate the `want` recipe from the test's output until the test runs green.
   2. If you receive a `DomainNotImplemented` error, then:
      1. Add the website to the [`pub fn parse_manually(&self, doc: &Html, url: &str) -> Result<Recipe>`](https://github.com/reaper47/recipya-rs/blob/main/crates/recipya-scraper/src/websites.rs) match statement.
      2. Create a file to implement the parser in the [respective folder](https://github.com/reaper47/recipya-rs/tree/main/crates/recipya-scraper/src/custom).
      3. Add the following skeleton:
        
        ```rust
        use scraper::{Html, Selector};
        
        use schema_org::{AtType, Recipe, at_context};
        
        use crate::Result;

        pub fn parse(doc: &Html, url: &str) -> Result<Recipe> {
            let root = &doc.root_element();
            
            Ok(Recipe {
                r#type: AtType::Recipe.to_opt(),
                context: at_context(),
                url: vec![url.into()],
                ..Default::default()
            })
        }
        ```

      4. Examine the recipe's HTML and add the missing fields. Refer to the other custom parsers for reference.
   3. Make the test pass.

#### Example

Follow these steps to verify/support https://www.allrecipes.com:

1. Open [websites-a.toml](https://github.com/reaper47/recipya-rs/blob/main/crates/recipya-scraper/data/websites-a.toml)
2. Append the website's metadata:
```text
[[website]]
name = "Allrecipes"
domain = "allrecipes.com"
variant = "AllRecipes"
url = "https://www.allrecipes.com/"
test = "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/"
```
3. Save the file 
4. Open [tests_scraper_a.rs](https://github.com/reaper47/recipya-rs/blob/main/crates/recipya-scraper/src/tests/tests_scraper_a.rs)
5. Add the following test:
```rust
#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_allrecipes_ok() -> Result<()> {
    let got = scrape(Website::AllRecipes, 0)?;

    pretty_assertions::assert_eq!(got, Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        url: vec!["https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into()],
        ..Default::default()
    });
    Ok(())
}
```
6. Run the test.
  - If there is no `DomainNotImplemented` error, then copy-paste the missing fields from the test's output to the test. Review the HTML to ensure no field has been forgotten.
  - If there is a `DomainNotImplemented`, then:
    1. Add the `Self::Allrecipes => custom::a::allrecipes::parse(doc, url),` arm to the [`pub fn parse_manually(&self, doc: &Html, url: &str) -> Result<Recipe>`](https://github.com/reaper47/recipya-rs/blob/main/crates/recipya-scraper/src/websites.rs) match statement.
    2. Define a `pub fn parse(doc: &Html, url: &str) -> Result<Recipe>` function in the [respective file](https://github.com/reaper47/recipya-rs/tree/main/crates/recipya-scraper/src/custom/a/allrecipes.rs)
    3. Implement the function using the [scraper crate](https://crates.io/crates/scraper) until the test passes.

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
