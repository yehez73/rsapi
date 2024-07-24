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

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = database::init_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone()) 
            .configure(config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
