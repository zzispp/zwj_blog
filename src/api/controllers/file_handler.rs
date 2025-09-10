
use actix_web::web;
use actix_multipart::form::{
        MultipartForm
    };

use crate::{
    api::dto::{file::UploadForm, response::ApiResponse},
    domain::{
        error::ApiError, models::file::Files, services::file::FileService
    },
};

pub async fn upload_file_handler(
    file_service: web::Data<dyn FileService>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<ApiResponse<Files>, ApiError> {
    let file_urls = file_service.upload_files(form.files).await?;
    Ok(ApiResponse::success(file_urls.into()))
}
