use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/user/all", web::get().to(crate::controller::user_controller::getall_users));
        // .route("/users", web::post().to(crate::controller::user_controller::create_user))
        // .route("/users/{id}", web::get().to(crate::controller::user_controller::get_user))
        // .route("/users/{id}", web::put().to(crate::controller::user_controller::update_user))
        // .route("/users/{id}", web::delete().to(crate::controller::user_controller::delete_user));
}   