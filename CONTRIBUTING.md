# Contributing

Thank you for your interest in contributing to Recipya! We look forward to your suggestions, ideas, bug reports and pull requests. Please feel free to ask questions and get help in the [discussion forum] before opening an issue or pull request.

Table of Contents
=================

* [Submitting bug reports](#submitting-bug-reports)
* [Submitting feature requests](#submitting-feature-requests)
* [Improve the documentation](#improve-the-documentation)
* [Translations](#translations)
* [Contributing code to Recipya](#contributing-code-to-recipya)
  * [Setting up Recipya locally](#setting-up-recipya-locally)
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
  * [How to support/verify a website](#how-to-supportverify-a-website)
      * [Example](#example)
  * [Coding style](#coding-style)
  * [Usage of AI](#usage-of-ai)
* [Support channels](#support-channels)

## Submitting bug reports

Please first browse through the [current issues] to see if the bug you want to report has already been reported. If not, open a [bug report].

[current issues]: https://github.com/reaper47/recipya-rs/issues?q=is%3Aissue%20label%3Abug
[bug report]: https://github.com/reaper47/recipya-rs/issues/new?template=bug_report.md

## Submitting feature requests

Have an idea on how to improve the software? Awesome. Please post your suggestion in the [discussion forum], or feel free to open a [feature request issue](https://github.com/reaper47/recipya-rs/issues/new?template=feature_request.md).

[discussion forum]: https://github.com/reaper47/recipya-rs/discussions

## Improve the documentation

To be detailed once the new documentation website will go live.

## Translations

To be detailed once the i18n system is implemented.

## Contributing code to Recipya

### Setting up Recipya locally

#### Linux

Follow these instructions to set up your development environment on Linux.

##### Tooling

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

##### Running Recipya

1. Clone the repository: `git clone https://github.com/reaper47/recipya-rs.git`
1. Open the repository in your IDE
1. Install the NPM dependencies: `cargo task web-install-deps`
1. Copy the `deploy/.env.example` file to the root of the repository and rename it to `.env`
1. Update the `DATABASE_URL` variable to point to your local Postgres database
1. Run the SQL migrations: `diesel migration run`
1. Run `cargo run server` to start the web server

#### Windows

Follow these instructions to set up your development environment on Windows if you do not use WSL nor Docker.

##### Tooling

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

##### Running Recipya

1. Clone the repository: `git clone https://github.com/reaper47/recipya-rs.git`
1. Open the repository in your IDE
1. Install the NPM dependencies: `cargo task web-install-deps`
1. Copy the `deploy\.env.example` file to the root of the repository and rename it to `.env`
1. Update the `DATABASE_URL` variable to point to your local Postgres database
1. Run the SQL migrations: `diesel migration run`
1. Run `cargo run server` to start the web server

#### Development Container

You may use the devcontainer to help develop Recipya. The `DATABASE_URL` environment variable in your
`.env` file would be `DATABASE_URL = "postgres://postgres:postgres@localhost:5432"`.

#### Useful Commands

##### Frontend

Run `cargo task web-install-deps` to install the NPM dependencies.

Run `cargo task web-build` to build the frontend.

##### Database

Run `diesel migration run` to run all pending migrations.

Run `diesel migration generate {migration_name}` to create a new SQL migration under [crates/repository/src/migrations](https://github.com/reaper47/recipya-rs/tree/main/crates/repository/src/migrations) when changes to the database need to be done.

Run `diesel migration redo` to rerun the latest migration.

Run `diesel migration redo --all` to rerun all migrations.

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

### Coding style

The [Rust Style Guide](https://github.com/rust-lang/rust/tree/HEAD/src/doc/style-guide/src) is used and enforced by [rustfmt](https://github.com/rust-lang/rustfmt).

Please run `cargo fmt` before opening a pull request to format all bin and lib files in the project.

### Usage of AI

There is a high probability your pull request will be rejected if the text and code is fully generated by AI to keep the human touch and avoid the project turning into AI slop. There is no reason to read something not created by a human.

This being said, you are free to use AI as long as you follow these rules:

* You write the pull request description and comments by hand. Grammatical/spelling mistakes are fine.
* You need to understand the code you are submitting.
* You write tests by hand to understand the domain.
* You test the changes manually.
* You either disclose the usage of generative AI in the pull request or in the code with `// AI usage:` comments.
* The usage of AI is banned for issues with the `good first issue` label because these are meant to learn.

It is vital that you keep your brain sharp. Offloading mentally-stimulating tasks will [reduce] your critical thinking capabilities.

[reduce]: https://www.apa.org/monitor/2026/07-08/ai-job-skills-thinking

## Support channels

You can also join our development and support channel on the [Matrix space: #recipya:matrix.org](https://app.element.io/#/room/#recipya:matrix.org). Matrix is similar to Discord but is open source.

You may also send an email to [reaper47](mailto:macpoule@gmail.com?subject=[GitHub]%20Recipya%20Inquiry).
