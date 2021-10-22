use actix_web::HttpResponse;

use super::ApplicationResponse;

/// Get(/health_check) returns a 200 to indicate the application is running
pub async fn analysis_http(site: String) -> ApplicationResponse {
    Ok(HttpResponse::Ok().body(format!("{}", site)))
}

/// Get(/health_check) returns a 200 to indicate the application is running
pub async fn analysis_https(site: String) -> ApplicationResponse {
    Ok(HttpResponse::Ok().body(format!("{}", site)))
}
