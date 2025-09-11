use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    models::blog::{Blog, CreateBlog, UpdateBlog},
    repositories::repository::{
        QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}

impl QueryParams for BlogQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn create(&self, new_blog: &CreateBlog) -> RepositoryResult<Blog>;
    async fn list(&self, params: BlogQueryParams) -> RepositoryResult<ResultPaging<Blog>>;
    async fn get(&self, blog_id: i32) -> RepositoryResult<Option<Blog>>;
    async fn get_by_slug(&self, slug: &str) -> RepositoryResult<Option<Blog>>;
    async fn get_published(&self) -> RepositoryResult<Vec<Blog>>;
    async fn get_published_by_slug(&self, slug: &str) -> RepositoryResult<Option<Blog>>;
    async fn update(&self, blog_id: i32, update_blog: &UpdateBlog) -> RepositoryResult<Option<Blog>>;
    async fn delete(&self, blog_id: i32) -> RepositoryResult<bool>;
    async fn exists(&self, blog_id: i32) -> RepositoryResult<bool>;
    async fn toggle_published(&self, blog_id: i32) -> RepositoryResult<Option<Blog>>;
}
