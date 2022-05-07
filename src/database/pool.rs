
use super::{ConnectionManager, Pool};

pub(crate) fn init_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}
