use actix_multipart::form::tempfile::TempFile;
use async_trait::async_trait;

use crate::domain::{error::CommonError, models::file::Files};


#[async_trait]
pub trait FileService: 'static + Sync + Send {
    async fn upload_files(&self, files: Vec<TempFile>) -> Result<Files, CommonError>;
}