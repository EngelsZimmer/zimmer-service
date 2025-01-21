use crate::errors::ZimmerError;
use crate::models::user::*;
use sqlx::postgres::PgPool;

pub async fn user_login_db(pool: &PgPool, login_user: LoginUser) -> Result<User, ZimmerError> {
    todo!()
}

pub async fn user_signup_db(pool: &PgPool, signup_user: SignupUser) -> Result<User, ZimmerError> {
    todo!()
}
