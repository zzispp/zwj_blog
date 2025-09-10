use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        ApiResponse {
            code: 200,
            message,
            data,
        }
    }
}

// 为ApiResponse实现Responder trait
impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
