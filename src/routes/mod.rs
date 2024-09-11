use actix_web::web::{self};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Division
        .route("/division/all", web::get().to(crate::controller::division_controller::getall_division))
        .route("/division/{id}", web::get().to(crate::controller::division_controller::get_specific_division))
        .route("/superadmin/division/add", web::post().to(crate::controller::division_controller::add_division))
        .route("/superadmin/division/update/{id}", web::put().to(crate::controller::division_controller::update_division))
        // Optional
        .route("/division/delete/{id}", web::delete().to(crate::controller::division_controller::delete_division))

        // Role
        .route("/role/all", web::get().to(crate::controller::role_controller::getall_role))
        .route("/role/{id}", web::get().to(crate::controller::role_controller::get_specific_role))
        .route("superadmin/role/add", web::post().to(crate::controller::role_controller::add_role))
        .route("/superadmin/role/update/{id}", web::put().to(crate::controller::role_controller::update_role))
        // Optional
        .route("/role/delete/{id}", web::delete().to(crate::controller::role_controller::delete_role))

        // Application
        .route("/application/all", web::get().to(crate::controller::application_controller::getall_application))
        .route("/application/{id}", web::get().to(crate::controller::application_controller::get_specific_application))
        .route("/superadmin/application/add", web::post().to(crate::controller::application_controller::add_application))
        .route("/superadmin/application/update/{id}", web::put().to(crate::controller::application_controller::update_application))
        // Optional
        .route("/application/delete/{id}", web::delete().to(crate::controller::application_controller::delete_application))

        // User Application Role
        .route("/user/application/role", web::get().to(crate::controller::user_application_role_controller::getall_userapplicationrole))

        // Application Role
        .route("/application/role/all", web::get().to(crate::controller::application_role_controller::getall_application_role))
        .route("/application/role/{id}", web::get().to(crate::controller::application_role_controller::get_specific_application_role))
        .route("/list/role/{id}", web::get().to(crate::controller::application_role_controller::list_application_role_by_id))
        .route("/superadmin/application/role/add", web::post().to(crate::controller::application_role_controller::add_application_role))
        .route("/superadmin/application/role/update/{id}", web::put().to(crate::controller::application_role_controller::update_application_role))
        .route("/superadmin/application/role/delete/{id}", web::put().to(crate::controller::application_role_controller::delete_application_role))

        // User
        .route("/superadmin/user/all", web::get().to(crate::controller::user_controller::getall_users))
        .route("/superadmin/user/add", web::post().to(crate::controller::user_controller::add_user))
        .route("/user/{id}", web::get().to(crate::controller::user_controller::get_specific_user))
        .route("/superadmin/user/delete/{id}", web::put().to(crate::controller::user_application_role_controller::delete_user_application_role))
        
        // Login
        .route("/login", web::post().to(crate::controller::auth_controller::login))
        .route("/logout", web::post().to(crate::controller::auth_controller::logout))

        // Profile
        .route("/auth/my/profile", web::get().to(crate::controller::profile_controller::my_profile))
        .route("/auth/change/password", web::put().to(crate::controller::auth_controller::change_password));
}