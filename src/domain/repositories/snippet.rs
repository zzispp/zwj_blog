use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    models::snippet::{CreateSnippet, Snippet, UpdateSnippet},
    repositories::repository::{
        QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnippetQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}

impl QueryParams for SnippetQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait SnippetRepository: Send + Sync {
    async fn create(&self, new_snippet: &CreateSnippet) -> RepositoryResult<Snippet>;
    async fn list(&self, params: SnippetQueryParams) -> RepositoryResult<ResultPaging<Snippet>>;
    async fn get(&self, snippet_id: i32) -> RepositoryResult<Option<Snippet>>;
    async fn get_by_slug(&self, slug: &str) -> RepositoryResult<Option<Snippet>>;
    async fn get_published(&self) -> RepositoryResult<Vec<Snippet>>;
    async fn update(
        &self,
        snippet_id: i32,
        update_snippet: &UpdateSnippet,
    ) -> RepositoryResult<Option<Snippet>>;
    async fn delete(&self, snippet_id: i32) -> RepositoryResult<bool>;
    async fn exists(&self, snippet_id: i32) -> RepositoryResult<bool>;
    async fn toggle_published(&self, snippet_id: i32) -> RepositoryResult<Option<Snippet>>;
}
