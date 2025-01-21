use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub user_email: String,
    pub user_password: String,
    pub user_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SignupUser {
    pub user_email: String,
    pub user_password: String,
    pub user_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub user_email: String,
    pub user_password: String,
}

impl From<web::Json<SignupUser>> for SignupUser {
    fn from(signup_user: web::Json<SignupUser>) -> Self {
        SignupUser {
            user_email: signup_user.user_email.clone(),
            user_password: signup_user.user_password.clone(),
            user_name: signup_user.user_name.clone(),
        }
    }
}

impl From<web::Json<LoginUser>> for LoginUser {
    fn from(login_user: web::Json<LoginUser>) -> Self {
        LoginUser {
            user_email: login_user.user_email.clone(),
            user_password: login_user.user_password.clone(),
        }
    }
}
