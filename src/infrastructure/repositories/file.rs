use actix_multipart::form::tempfile::TempFile;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::{
    domain::{
        models::file::{CreateFile, File},
        repositories::{file::FileRepository, repository::RepositoryResult},
    },
    infrastructure::{
        databases::postgresql::DBConn,
        error::DieselRepositoryError,
        models::file::{CreateFileDiesel, FileDiesel},
    },
};

pub struct FileDieselRepository {
    pub pool: Arc<DBConn>,
}

impl FileDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        FileDieselRepository { pool: db }
    }
}

#[async_trait]
impl FileRepository for FileDieselRepository {
    async fn find_by_hash(&self, file_hash: &str) -> RepositoryResult<Option<File>> {
        use crate::infrastructure::schema::files::dsl::{file_hash as hash_col, files};
        let pool = self.pool.clone();
        let hash = file_hash.to_string();

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            files
                .filter(hash_col.eq(hash))
                .first::<FileDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.map(|v| v.into()))
    }

    async fn create(&self, new_file: &CreateFile) -> RepositoryResult<File> {
        use crate::infrastructure::schema::files::dsl::files;
        let new_file_diesel: CreateFileDiesel = CreateFileDiesel::from(new_file.clone());
        let mut conn = self.pool.get().unwrap();

        let result: FileDiesel = run(move || {
            diesel::insert_into(files)
                .values(new_file_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into())
    }

    async fn save_files(&self, files: Vec<TempFile>) -> RepositoryResult<HashMap<String, String>> {
        if files.is_empty() {
            return Ok(HashMap::new());
        }

        // 提前创建存储目录，只创建一次
        let upload_dir = "uploads";
        tokio::fs::create_dir_all(upload_dir)
            .await
            .map_err(|e| crate::domain::error::RepositoryError::DatabaseError(e.to_string()))?;

        let mut file_urls = HashMap::new();

        for file in files {
            // 获取原始文件名
            let original_filename = file
                .file_name
                .clone()
                .unwrap_or_else(|| "unknown".to_string());

            // 流式计算hash，避免一次性读取整个文件到内存
            let mut hasher = md5::Context::new();
            let mut file_reader = tokio::fs::File::open(&file.file.path())
                .await
                .map_err(|e| crate::domain::error::RepositoryError::DatabaseError(e.to_string()))?;

            let mut buffer = [0; 8192]; // 8KB缓冲区
            use tokio::io::AsyncReadExt;
            loop {
                let bytes_read = file_reader.read(&mut buffer).await.map_err(|e| {
                    crate::domain::error::RepositoryError::DatabaseError(e.to_string())
                })?;
                if bytes_read == 0 {
                    break;
                }
                hasher.consume(&buffer[..bytes_read]);
            }

            let file_hash = format!("{:x}", hasher.finalize());

            // 检查数据库中是否已存在该hash的文件
            if let Some(existing_file) = self.find_by_hash(&file_hash).await? {
                // 文件已存在，直接使用数据库中的路径
                file_urls.insert(original_filename, existing_file.file_path);
                continue;
            }

            // 获取文件扩展名
            let extension = std::path::Path::new(&original_filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| format!(".{}", ext))
                .unwrap_or_default();

            // 使用hash作为文件名
            let unique_filename = format!("{}{}", file_hash, extension);
            let file_path = format!("{}/{}", upload_dir, unique_filename);
            let db_file_path = format!("/static/{}", unique_filename); // 不带host的路径

            // 保存文件到磁盘
            file.file
                .persist(&file_path)
                .map_err(|e| crate::domain::error::RepositoryError::DatabaseError(e.to_string()))?;

            // 保存文件记录到数据库
            let create_file = CreateFile {
                file_hash,
                file_path: db_file_path.clone(),
            };
            let _file_record = self.create(&create_file).await?;

            file_urls.insert(original_filename, db_file_path);
        }

        Ok(file_urls)
    }
}
