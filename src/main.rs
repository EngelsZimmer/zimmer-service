use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;

#[path = "./auth/mod.rs"]
mod auth;
#[path = "./dbaccess/mod.rs"]
mod dbaccess;
#[path = "./errors.rs"]
mod errors;
#[path = "./handlers/mod.rs"]
mod handlers;
#[path = "./models/mod.rs"]
mod models;
#[path = "./routes.rs"]
mod routes;
#[path = "./state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "Zimmer is running on port 8000".to_string(),
        db: db_pool,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // 모든 도메인 허용 (개발용)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .app_data(shared_data.clone())
            .configure(general_route)
            .configure(user_route)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
