use std::collections::HashMap;

use actix_multipart::form::tempfile::TempFile;
use async_trait::async_trait;

use crate::domain::{
    models::file::{CreateFile, File},
    repositories::repository::RepositoryResult,
};

#[async_trait]
pub trait FileRepository: Send + Sync {
    // 上传文件并返回文件名和URL的映射
    async fn save_files(&self, files: Vec<TempFile>) -> RepositoryResult<HashMap<String, String>>;

    // 根据hash查找文件记录
    async fn find_by_hash(&self, file_hash: &str) -> RepositoryResult<Option<File>>;

    // 创建新的文件记录
    async fn create(&self, new_file: &CreateFile) -> RepositoryResult<File>;
}
