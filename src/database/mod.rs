use crate::errors::ServiceError;

pub mod pool;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub fn db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {
    pool.get().map_err(|_| ServiceError::UnableToConnectToDb)
}
