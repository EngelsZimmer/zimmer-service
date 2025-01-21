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
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;

    #[actix_web::test]
    async fn user_login_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file.");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: db_pool,
        });
        let user = web::Json(LoginUser {
            user_email: "user@example.com".to_string(),
            user_password: "hashed_password_example".to_string(),
        });
        let resp = user_login(app_state, user).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn user_signup_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file.");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: db_pool,
        });
        let user = web::Json(SignupUser {
            user_email: "user2@example.com".to_string(),
            user_password: "1234".to_string(),
            user_name: "user2".to_string(),
        });
        let resp = user_signup(app_state, user).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
