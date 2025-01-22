use crate::errors::ZimmerError;
use crate::models::user::*;
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn login_db(pool: &PgPool, login_user: LoginUser) -> Result<User, ZimmerError> {
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

pub async fn signup_db(pool: &PgPool, signup_user: SignupUser) -> Result<User, ZimmerError> {
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

pub async fn update_user_db(
    pool: &PgPool,
    update_user: UpdateUser,
    id: Uuid,
) -> Result<User, ZimmerError> {
    let current_user = sqlx::query!(
        r#"
        SELECT * FROM users WHERE user_id=$1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| ZimmerError::NotFound("No such user".into()))?;

    let name = if let Some(name) = update_user.user_name {
        name
    } else {
        current_user.user_name
    };

    let user_row = sqlx::query!(
        r#"
        UPDATE users SET user_name=$1
        WHERE user_id=$2
        returning user_id, user_email, user_password, user_name
        "#,
        name,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| ZimmerError::DBError(format!("DBError occured: {}", err).into()))?;

    Ok(User {
        user_id: user_row.user_id,
        user_email: user_row.user_email,
        user_password: user_row.user_password,
        user_name: user_row.user_name,
    })
}

pub async fn delete_user_db(pool: &PgPool, id: Uuid) -> Result<String, ZimmerError> {
    let resp = sqlx::query!(
        r#"
        DELETE FROM users WHERE user_id=$1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:#?} record", resp))
}
