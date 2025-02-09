use diesel::{Connection, ConnectionError, ConnectionResult};
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures_util::FutureExt;
use native_tls::Certificate;
use tokio_postgres::NoTls;

pub type PgConn = AsyncPgConnection;
pub type PgPooledConn<'a> = bb8::PooledConnection<'a, PgConn>;
pub type PgPool = bb8::Pool<PgConn>;

/// The path to the migration files to embed into the binary.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/core/repository/migrations");

/// A wrapper around a PostgreSQL connection pool (`PgPool`).
#[derive(Clone)]
pub struct DbPool(pub PgPool);

impl DbPool {
    /// Asynchronously retrieves a pooled connection from the pool.
    pub async fn get(&self) -> Result<PgPooledConn, bb8::RunError> {
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

/// Establishes a connection to the PostgreSQL database and applies migrations.
async fn establish(database_url: &str) -> ConnectionResult<AsyncPgConnection> {
    diesel::PgConnection::establish(database_url)
        .expect("error connecting to database")
        .run_pending_migrations(MIGRATIONS)
        .expect("migrations should have been applied");

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
                tracing::error!("connection error: {err}");
            }
        });

        AsyncPgConnection::try_from(client).await
    } else {
        let pg_cert = include_bytes!("../../../pg_cert.pem");
        let certificate = Certificate::from_pem(pg_cert).map_err(|e| {
            ConnectionError::BadConnection(format!("Error reading certificate: {}", e))
        })?;

        let tls_connector = native_tls::TlsConnector::builder()
            .add_root_certificate(certificate)
            .build()
            .map_err(|err| ConnectionError::BadConnection(format!("Error building TLS: {err}")))?;
        let pg_tls = postgres_native_tls::MakeTlsConnector::new(tls_connector);

        let (client, connection) = tokio_postgres::connect(database_url, pg_tls)
            .await
            .map_err(|err| {
                ConnectionError::BadConnection(format!("Error connecting to Postgres: {err}"))
            })?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                tracing::error!("connection error: {err}");
            }
        });

        AsyncPgConnection::try_from(client).await
    }
}
