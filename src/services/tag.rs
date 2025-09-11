use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    error::CommonError,
    models::tag::{CreateTag, Tag, TagType},
    repositories::{
        repository::ResultPaging,
        tag::{TagQueryParams, TagRepository},
    },
    services::tag::TagService,
};

#[derive(Clone)]
pub struct TagServiceImpl {
    pub repository: Arc<dyn TagRepository>,
}

impl TagServiceImpl {
    pub fn new(repository: Arc<dyn TagRepository>) -> Self {
        TagServiceImpl { repository }
    }
}

#[async_trait]
impl TagService for TagServiceImpl {
    /// 创建标签
    async fn create(&self, tag: CreateTag) -> Result<Tag, CommonError> {
        self.repository
            .create(&tag)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: TagQueryParams) -> Result<ResultPaging<Tag>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, tag_id: i32) -> Result<Option<Tag>, CommonError> {
        self.repository
            .get(tag_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, tag_id: i32) -> Result<bool, CommonError> {
        self.repository
            .delete(tag_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn exists(&self, tag_id: i32) -> Result<bool, CommonError> {
        self.repository
            .exists(tag_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn update(
        &self,
        tag_id: i32,
        update_tag: crate::domain::models::tag::UpdateTag,
    ) -> Result<Option<Tag>, CommonError> {
        self.repository
            .update(tag_id, &update_tag)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_all(&self, tag_type: Option<TagType>) -> Result<Vec<Tag>, CommonError> {
        self.repository
            .get_all(tag_type)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
