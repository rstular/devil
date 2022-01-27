use crate::{get_config_reader, get_settings_reader};
use diesel::sqlite::SqliteConnection;
use log::{debug, error};
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub mod models;
pub mod schema;

embed_migrations!();

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn run_migrations(conn: &SqliteConnection) {
    match diesel_migrations::run_pending_migrations(conn) {
        Ok(_) => debug!("Migrations ran successfully"),
        Err(e) => error!("Error running migrations: {}", e),
    };
}

pub fn establish_connection() -> DbPool {
    let database_url = &get_settings_reader().db_path;
    debug!("Connecting to database at {}", database_url);

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap_or_else(|e| {
        error!("Failed to create database pool: {}", e);
        std::process::abort();
    });

    if cfg!(test) || get_config_reader().get_bool("db-migrate").unwrap_or(false) {
        run_migrations(&pool.get().expect("Failed to acquire database connection"));
    }

    pool
}
