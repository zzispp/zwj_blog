use crate::domain::models::file::{CreateFile, File};
use crate::infrastructure::schema::files;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct FileDiesel {
    pub id: i32,
    pub file_hash: String,
    pub file_path: String,
    pub upload_time: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct CreateFileDiesel {
    pub file_hash: String,
    pub file_path: String,
}

// 转换方法
impl From<File> for FileDiesel {
    fn from(f: File) -> Self {
        FileDiesel {
            id: f.id,
            file_hash: f.file_hash,
            file_path: f.file_path,
            upload_time: f.upload_time,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

impl Into<File> for FileDiesel {
    fn into(self) -> File {
        File {
            id: self.id,
            file_hash: self.file_hash,
            file_path: self.file_path,
            upload_time: self.upload_time,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl From<CreateFile> for CreateFileDiesel {
    fn from(f: CreateFile) -> Self {
        CreateFileDiesel {
            file_hash: f.file_hash,
            file_path: f.file_path,
        }
    }
}
