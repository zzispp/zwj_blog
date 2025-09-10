use actix_web::web;

use crate::{
    api::dto::{response::ApiResponse, user::GetNonceDTO},
    domain::{error::ApiError, services::user::UserService},
};

pub async fn get_nonce_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<GetNonceDTO>,
) -> Result<ApiResponse<String>, ApiError> {
    let address = post_data.into_inner().address;
    let nonce = user_service
        .get_nonce(address)
        .await?;
    Ok(ApiResponse::success(nonce))
}
