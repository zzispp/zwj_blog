use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    models::tag::{CreateTag, Tag, TagType, UpdateTag},
    repositories::repository::{
        QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub tag_type: Option<TagType>,
}

impl QueryParams for TagQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn create(&self, new_tag: &CreateTag) -> RepositoryResult<Tag>;
    async fn list(&self, params: TagQueryParams) -> RepositoryResult<ResultPaging<Tag>>;
    async fn get(&self, tag_id: i32) -> RepositoryResult<Option<Tag>>;
    async fn update(&self, tag_id: i32, update_tag: &UpdateTag) -> RepositoryResult<Option<Tag>>;
    async fn delete(&self, tag_id: i32) -> RepositoryResult<bool>;
    async fn exists(&self, tag_id: i32) -> RepositoryResult<bool>;
    async fn get_all(&self, tag_type: Option<TagType>) -> RepositoryResult<Vec<Tag>>;
}
