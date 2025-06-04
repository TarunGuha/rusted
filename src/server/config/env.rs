use log::warn;
use once_cell::sync::Lazy;
use std::env;

pub static REDIS_URL: Lazy<String> = Lazy::new(|| match env::var("REDIS_URL") {
    Ok(val) => val,
    Err(_) => {
        let default = "redis://127.0.0.1:6379".to_string();
        warn!("REDIS_URL Not Set. Using default: {}", default);
        default
    }
});
