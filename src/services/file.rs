use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::file::{FileInfo, Files},
    repositories::file::FileRepository,
    services::file::FileService,
};

#[derive(Clone)]
pub struct FileServiceImpl {
    pub repository: Arc<dyn FileRepository>,
}

impl FileServiceImpl {
    pub fn new(repository: Arc<dyn FileRepository>) -> Self {
        FileServiceImpl { repository }
    }
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn upload_files(&self, files: Vec<TempFile>) -> Result<Files, CommonError> {
        let files_info = self
            .repository
            .save_files(files)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        let mut files: Vec<FileInfo> = Vec::new();
        for (file_name, url) in files_info {
            files.push(FileInfo { file_name, url });
        }
        Ok(Files { files })
    }
}
