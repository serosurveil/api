use actix_web::HttpResponse;

use super::ApplicationResponse;

/// Get(/health_check) returns a 200 to indicate the application is running
pub async fn health_check() -> ApplicationResponse {
    Ok(HttpResponse::Ok().finish())
}

pub async fn say_hello() -> ApplicationResponse {
    Ok(HttpResponse::Ok().body("Hello, there"))
}
