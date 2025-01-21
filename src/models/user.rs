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
pub struct CreateUser {
    pub user_email: String,
    pub user_password: String,
    pub user_name: String,
}

impl From<web::Json<CreateUser>> for CreateUser {
    fn from(new_user: web::Json<CreateUser>) -> Self {
        CreateUser {
            user_email: new_user.user_email.clone(),
            user_password: new_user.user_password.clone(),
            user_name: new_user.user_name.clone(),
        }
    }
}
