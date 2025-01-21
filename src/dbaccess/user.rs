use crate::errors::ZimmerError;
use crate::models::user::*;
use sqlx::postgres::PgPool;

pub async fn user_login_db(pool: &PgPool, login_user: LoginUser) -> Result<User, ZimmerError> {
    let user_row = sqlx::query!(
        r#"SELECT *
        FROM users
        WHERE user_email=$1 AND user_password=$2"#,
        login_user.user_email,
        login_user.user_password
    )
    .fetch_one(pool)
    .await
    .map(|user| User {
        user_id: user.user_id,
        user_email: user.user_email,
        user_password: user.user_password,
        user_name: user.user_name,
    })
    .map_err(|_err| ZimmerError::NotFound("Wrong email or password".into()))?;

    Ok(user_row)
}

pub async fn user_signup_db(pool: &PgPool, signup_user: SignupUser) -> Result<User, ZimmerError> {
    let user_row = sqlx::query!(
        r#"
        INSERT INTO users (user_email, user_password, user_name)
        VALUES ($1, $2, $3)
        returning user_id, user_email, user_password, user_name
        "#,
        signup_user.user_email,
        signup_user.user_password,
        signup_user.user_name
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        user_id: user_row.user_id,
        user_email: user_row.user_email,
        user_password: user_row.user_password,
        user_name: user_row.user_name,
    })
}
