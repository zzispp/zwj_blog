use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::tag::{CreateTag, Tag},
    repositories::{repository::ResultPaging, tag::TagQueryParams},
};

#[async_trait]
pub trait TagService: 'static + Sync + Send {
    async fn create(&self, tag: CreateTag) -> Result<Tag, CommonError>;
    async fn list(&self, params: TagQueryParams) -> Result<ResultPaging<Tag>, CommonError>;
    async fn get(&self, tag_id: i32) -> Result<Option<Tag>, CommonError>;
    async fn update(
        &self,
        tag_id: i32,
        update_tag: crate::domain::models::tag::UpdateTag,
    ) -> Result<Option<Tag>, CommonError>;
    async fn delete(&self, tag_id: i32) -> Result<bool, CommonError>;
    async fn exists(&self, tag_id: i32) -> Result<bool, CommonError>;
    async fn get_all(
        &self,
        tag_type: Option<crate::domain::models::tag::TagType>,
    ) -> Result<Vec<Tag>, CommonError>;
}
