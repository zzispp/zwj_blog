use actix_web::web;

use crate::{
    api::dto::{
        response::ApiResponse,
        user::{GetNonceDTO, VerifySignatureDTO},
    },
    domain::{
        error::ApiError,
        services::user::{AuthResponse, UserService},
    },
};

pub async fn get_nonce_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<GetNonceDTO>,
) -> Result<ApiResponse<String>, ApiError> {
    let address = post_data.into_inner().address;
    let nonce = user_service.get_nonce(address).await?;
    Ok(ApiResponse::success(nonce))
}

//验证签名并返回JWT token
pub async fn verify_signature_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<VerifySignatureDTO>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    let VerifySignatureDTO { address, signature } = post_data.into_inner();
    let result = user_service.verify_signature(address, signature).await?;
    Ok(ApiResponse::success(result))
}
