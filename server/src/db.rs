use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::sqlite::SqliteConnection;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}
