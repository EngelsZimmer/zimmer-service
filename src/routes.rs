use crate::handlers::{general::*, user::*};
use actix_web::web;

pub fn general_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check_handler));
}

pub fn user_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/login", web::post().to(login))
            .route("/signup", web::post().to(signup))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user)),
    );
}
