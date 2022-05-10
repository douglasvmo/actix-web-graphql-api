pub mod pool;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub fn db_connection(pool: &Pool) -> PooledConnection {
  pool.get().expect("Unable to connect to DB")
}