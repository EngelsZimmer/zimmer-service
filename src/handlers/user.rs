use crate::dbaccess::user::*;
use crate::errors::ZimmerError;
use crate::models::user::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn login(
    app_state: web::Data<AppState>,
    user: web::Json<LoginUser>,
) -> Result<HttpResponse, ZimmerError> {
    login_db(&app_state.db, user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn signup(
    app_state: web::Data<AppState>,
    user: web::Json<SignupUser>,
) -> Result<HttpResponse, ZimmerError> {
    signup_db(&app_state.db, user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn update_user(
    app_state: web::Data<AppState>,
    user: web::Json<UpdateUser>,
    params: web::Path<Uuid>,
) -> Result<HttpResponse, ZimmerError> {
    let user_id = params.into_inner();
    update_user_db(&app_state.db, user.into(), user_id)
        .await
        .map(|updated_user| HttpResponse::Ok().json(updated_user))
}

pub async fn delete_user(
    app_state: web::Data<AppState>,
    params: web::Path<Uuid>,
) -> Result<HttpResponse, ZimmerError> {
    let user_id = params.into_inner();
    delete_user_db(&app_state.db, user_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;

    #[actix_web::test]
    async fn login_success() {
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
        let resp = login(app_state, user).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn signup_success() {
        // 데이터베이스 준비
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file.");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        // 롤백 준비
        let tx = db_pool.begin().await.unwrap();

        // 테스트 함수 파라미터 준비
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: db_pool,
        });
        let user = web::Json(SignupUser {
            user_email: "user3@example.com".to_string(),
            user_password: "1234".to_string(),
            user_name: "user3".to_string(),
        });

        // 테스트 함수 실행
        let resp = signup(app_state, user).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // 롤백
        tx.rollback().await.unwrap();
    }
}
