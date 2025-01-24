use crate::auth::middleware::jwt_auth_middleware;
use crate::handlers::{general::*, user::*};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn general_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check_handler));
}

pub fn user_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/login", web::post().to(login))
            .route("/signup", web::post().to(signup)),
    );

    cfg.service(
        web::scope("/protected/user")
            .wrap(HttpAuthentication::bearer(jwt_auth_middleware))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user)),
    );
}
