use crate::dbaccess::user::*;
use crate::errors::ZimmerError;
use crate::models::user::*;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn user_login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn user_signup() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod test {}
