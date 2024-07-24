use actix_web::{App, HttpServer, middleware::Logger};
use crate::routes::config_routes;
use env_logger::{Builder, Env};

mod middleware;
mod routes;
mod controller;
mod service;
mod models;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
