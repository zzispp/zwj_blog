use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub file_hash: String,
    pub file_path: String,
    pub upload_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct CreateFile {
    pub file_hash: String,
    pub file_path: String,
}

// API响应用的文件信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Files {
    pub files: Vec<FileInfo>,
}
