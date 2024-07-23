use actix_web::{App, HttpServer};
use crate::routes::config_routes;

mod middleware;
mod routes;
mod controller;
mod service;
mod models;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
