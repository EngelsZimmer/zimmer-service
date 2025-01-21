use crate::dbaccess::user::*;
use crate::errors::ZimmerError;
use crate::models::user::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn user_login(
    app_state: web::Data<AppState>,
    user: web::Json<LoginUser>,
) -> Result<HttpResponse, ZimmerError> {
    user_login_db(&app_state.db, user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn user_signup(
    app_state: web::Data<AppState>,
    signup_user: web::Json<SignupUser>,
) -> Result<HttpResponse, ZimmerError> {
    user_signup_db(&app_state.db, signup_user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

#[cfg(test)]
mod test {}
