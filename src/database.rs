use diesel::pg::PgConnection;
use diesel::r2d2::PooledConnection;
use r2d2::Pool;

type ConnectionManager = diesel::r2d2::ConnectionManager<PgConnection>;

pub type PoolConnection = Pool<ConnectionManager>;

pub type Connection = PooledConnection<ConnectionManager>;

pub(crate) fn init_pool() -> PoolConnection {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let mgr = ConnectionManager::new(database_url);

    Pool::builder().build(mgr).expect("Failed to create pool")
}
