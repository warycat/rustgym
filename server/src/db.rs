use actix_web::error::ErrorInternalServerError;
use actix_web::web;
use actix_web::Error;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PoolError;
use diesel::r2d2::PooledConnection;
use diesel::sqlite::SqliteConnection;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn conn(pool: web::Data<SqlitePool>) -> Result<SqlitePooledConnection, Error> {
    pool.get().map_err(ErrorInternalServerError)
}
