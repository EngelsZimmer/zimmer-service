use super::jwt::verify_jwt;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use dotenv::dotenv;
use std::env;

pub async fn jwt_auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file.");
    let token = credentials.token();

    match verify_jwt(token, &secret_key) {
        Ok(claims) => {
            // 요청에 클레임 추가 (필요하면 사용)
            req.extensions_mut().insert(claims);
            Ok(req) // 인증 성공 시 요청 계속 진행
        }
        Err(_err) => {
            let error = actix_web::error::ErrorUnauthorized("Invalid or expired token");
            Err((error, req)) // 인증 실패
        }
    }
}
