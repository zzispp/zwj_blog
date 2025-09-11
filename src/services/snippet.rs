use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    error::CommonError,
    models::snippet::{CreateSnippet, Snippet, UpdateSnippet},
    repositories::{
        repository::ResultPaging,
        snippet::{SnippetQueryParams, SnippetRepository},
    },
    services::snippet::SnippetService,
};

#[derive(Clone)]
pub struct SnippetServiceImpl {
    pub repository: Arc<dyn SnippetRepository>,
}

impl SnippetServiceImpl {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        SnippetServiceImpl { repository }
    }
}

#[async_trait]
impl SnippetService for SnippetServiceImpl {
    async fn create(&self, snippet: CreateSnippet) -> Result<Snippet, CommonError> {
        self.repository
            .create(&snippet)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: SnippetQueryParams) -> Result<ResultPaging<Snippet>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, snippet_id: i32) -> Result<Option<Snippet>, CommonError> {
        self.repository
            .get(snippet_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Snippet>, CommonError> {
        self.repository
            .get_by_slug(slug)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_published(&self) -> Result<Vec<Snippet>, CommonError> {
        self.repository
            .get_published()
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn update(
        &self,
        snippet_id: i32,
        update_snippet: UpdateSnippet,
    ) -> Result<Option<Snippet>, CommonError> {
        self.repository
            .update(snippet_id, &update_snippet)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, snippet_id: i32) -> Result<bool, CommonError> {
        self.repository
            .delete(snippet_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn exists(&self, snippet_id: i32) -> Result<bool, CommonError> {
        self.repository
            .exists(snippet_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn toggle_published(&self, snippet_id: i32) -> Result<Option<Snippet>, CommonError> {
        self.repository
            .toggle_published(snippet_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
