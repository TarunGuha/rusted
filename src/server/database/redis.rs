use crate::server::config::env::REDIS_URL;
use deadpool_redis::{Config, Pool, Runtime};

pub fn create_redis_pool() -> Pool {
    let redis_url = REDIS_URL.clone();

    let mut cfg = Config::from_url(redis_url);

    // Ensure pool config exists, then set max_size
    cfg.pool.get_or_insert(Default::default()).max_size = 500;

    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool")
}
