// Snippet service trait - placeholder
use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::snippet::{CreateSnippet, Snippet, UpdateSnippet},
    repositories::{repository::ResultPaging, snippet::SnippetQueryParams},
};

#[async_trait]
pub trait SnippetService: 'static + Sync + Send {
    async fn create(&self, snippet: CreateSnippet) -> Result<Snippet, CommonError>;
    async fn list(&self, params: SnippetQueryParams) -> Result<ResultPaging<Snippet>, CommonError>;
    async fn get(&self, snippet_id: i32) -> Result<Option<Snippet>, CommonError>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Snippet>, CommonError>;
    async fn get_published(&self) -> Result<Vec<Snippet>, CommonError>;
    async fn update(
        &self,
        snippet_id: i32,
        update_snippet: UpdateSnippet,
    ) -> Result<Option<Snippet>, CommonError>;
    async fn delete(&self, snippet_id: i32) -> Result<bool, CommonError>;
    async fn exists(&self, snippet_id: i32) -> Result<bool, CommonError>;
    async fn toggle_published(&self, snippet_id: i32) -> Result<Option<Snippet>, CommonError>;
}
