use actix_web::{web::{self, route}, HttpRequest};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Division
        .route("/division/all", web::get().to(crate::controller::division_controller::getall_division))
        .route("/division/add", web::post().to(crate::controller::division_controller::add_division))
        .route("/division/update/{id}", web::put().to(crate::controller::division_controller::update_division))
        .route("/division/delete/{id}", web::delete().to(crate::controller::division_controller::delete_division))

        // Role
        .route("/role/all", web::get().to(crate::controller::role_controller::getall_role))
        .route("/role/add", web::post().to(crate::controller::role_controller::add_role))
        .route("/role/update/{id}", web::put().to(crate::controller::role_controller::update_role))
        .route("/role/delete/{id}", web::delete().to(crate::controller::role_controller::delete_role))

        // Application
        .route("/application/all", web::get().to(crate::controller::application_controller::getall_application))
        .route("/application/add", web::post().to(crate::controller::application_controller::add_application))
        .route("/application/update/{id}", web::put().to(crate::controller::application_controller::update_application))
        .route("/application/delete/{id}", web::delete().to(crate::controller::application_controller::delete_application))

        // Application Role
        .route("/user/application/role", web::get().to(crate::controller::user_application_role_controller::getall_userapplicationrole))

        // User
        .route("/user/all", web::get().to(crate::controller::user_controller::getall_users))
        .route("/user/add", web::post().to(crate::controller::user_controller::add_user))
        // .route("/users/{id}", web::get().to(crate::controller::user_controller::get_user))
        // .route("/users/{id}", web::put().to(crate::controller::user_controller::update_user))
        // .route("/users/{id}", web::delete().to(crate::controller::user_controller::delete_user));
        
        // Login
        .route("/login", web::post().to(crate::controller::login_controller::login))

        // Profile
        .route("/auth/my/profile", web::get().to(crate::controller::profile_controller::my_profile));
}