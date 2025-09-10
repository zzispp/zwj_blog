use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl From<&str> for CommonError {
    fn from(error: &str) -> CommonError {
        CommonError { message: error.to_string(), code: 500 }
    }
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().json(&self.0)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(String),
    SerializationError(String),
    NotFound(String),
    ValidationError(String),
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        let message = match self {
            RepositoryError::DatabaseError(msg) => format!("数据库错误: {}", msg),
            RepositoryError::SerializationError(msg) => format!("序列化错误: {}", msg),
            RepositoryError::NotFound(msg) => format!("未找到: {}", msg),
            RepositoryError::ValidationError(msg) => format!("验证错误: {}", msg),
        };
        CommonError {
            message,
            code: 500,
        }
    }
}
