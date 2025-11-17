use std::sync::Arc;

use diesel::sql_types::{BigInt, Text};
use diesel::{
    Connection, ConnectionError, ConnectionResult, QueryableByName, RunQueryDsl, sql_query,
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig, bb8};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use futures_util::FutureExt;
use postgres_rustls::MakeTlsConnector;
use rustls::pki_types::CertificateDer;
use rustls::{ClientConfig, RootCertStore};
use tokio_postgres::{Config, NoTls};
use tokio_rustls::TlsConnector;
use tracing::{error, info, warn};

pub type PgConn = AsyncPgConnection;
pub type PgPooledConn<'a> = bb8::PooledConnection<'a, PgConn>;
pub type PgPool = bb8::Pool<PgConn>;

/// The path to the migration files to embed into the binary.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

/// A wrapper around a PostgreSQL connection pool (`PgPool`).
#[derive(Clone)]
pub struct DbPool(pub PgPool);

impl DbPool {
    /// Asynchronously retrieves a pooled connection from the pool.
    pub async fn get(&self) -> Result<PgPooledConn<'_>, bb8::RunError> {
        self.0.get().await
    }
}

/// Creates and initializes a PostgreSQL connection pool.
pub async fn make_db_pool(database_url: &str) -> Result<DbPool, bb8::RunError> {
    let mut config = ManagerConfig::<PgConn>::default();
    config.custom_setup = Box::new(|url| establish(url).boxed());

    let manager = AsyncDieselConnectionManager::<PgConn>::new_with_config(database_url, config);
    let pool = bb8::Pool::builder().build(manager).await?;
    Ok(DbPool(pool))
}

#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

/// Creates a new database if it doesn't already exist.
pub fn create_database_if_not_exists(db_name: &str) -> Result<(), diesel::result::Error> {
    let db_url = std::env::var("RECIPYA_DATABASE_URL")
        .expect("Environment variable 'RECIPYA_DATABASE_URL' not set");

    let conn = &mut diesel::PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {db_url}"));

    let res = sql_query("SELECT COUNT(*) as count FROM pg_database WHERE datname = $1")
        .bind::<Text, _>(db_name)
        .get_result::<CountResult>(conn);

    match res {
        Ok(row) if row.count > 0 => {
            info!("Database '{}' exists", db_name);
            Ok(())
        }
        Ok(_) => {
            warn!("Database '{db_name}' does not exist, creating it");

            sql_query(format!("CREATE DATABASE {db_name}"))
                .execute(conn)
                .map(|_| ())
                .map_err(|err| {
                    error!("Failed to create database '{db_name}': {err:?}");
                    err
                })
        }
        Err(err) => {
            error!("Error checking database existence: {err:?}");
            Err(err)
        }
    }
}

/// Establishes a connection to the PostgreSQL database and applies migrations.
async fn establish(database_url: &str) -> ConnectionResult<AsyncPgConnection> {
    diesel::PgConnection::establish(database_url)
        .expect("error connecting to database")
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|err| panic!("migrations should have been applied for {database_url}: {err:?}"));

    if database_url.contains("localhost") || database_url.contains("host.docker.internal") {
        let (client, connection) =
            tokio_postgres::connect(database_url, NoTls)
                .await
                .map_err(|err| {
                    ConnectionError::BadConnection(format!(
                        "Error connection to {database_url}: {err}"
                    ))
                })?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                error!("connection error: {err}");
            }
        });

        AsyncPgConnection::try_from(client).await
    } else {
        let pg_cert = include_bytes!("../../../pg_cert.pem");

        let mut root_cert_store = RootCertStore::empty();
        root_cert_store.add_parsable_certificates([CertificateDer::from(pg_cert.as_ref())]);

        let client_config = ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(client_config));
        let tls = MakeTlsConnector::new(connector);

        let config: Config = database_url
            .parse()
            .map_err(|err| ConnectionError::BadConnection(format!("Invalid DB URL: {err}")))?;

        let (client, connection) = config.connect(tls).await.map_err(|err| {
            ConnectionError::BadConnection(format!("Error connecting to Postgres: {err}"))
        })?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                error!("connection error: {err}");
            }
        });

        AsyncPgConnection::try_from(client).await
    }
}
