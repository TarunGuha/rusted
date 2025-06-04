use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;

mod server;
mod services;

use server::database::redis::create_redis_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let redis_pool = create_redis_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))
            .wrap(Logger::default())
            .service(services::auth::router::auth_router())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
