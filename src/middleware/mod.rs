use actix_cors::Cors;

pub fn cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin("http://127.0.0.1:4200")
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .supports_credentials()
        .max_age(3600)
}
