use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    models::note::{CreateNote, Note, UpdateNote},
    repositories::repository::{
        QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}

impl QueryParams for NoteQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn create(&self, new_note: &CreateNote) -> RepositoryResult<Note>;
    async fn list(&self, params: NoteQueryParams) -> RepositoryResult<ResultPaging<Note>>;
    async fn get(&self, note_id: i32) -> RepositoryResult<Option<Note>>;
    async fn get_all(&self) -> RepositoryResult<Vec<Note>>;
    async fn update(
        &self,
        note_id: i32,
        update_note: &UpdateNote,
    ) -> RepositoryResult<Option<Note>>;
    async fn delete(&self, note_id: i32) -> RepositoryResult<bool>;
    async fn exists(&self, note_id: i32) -> RepositoryResult<bool>;
    async fn toggle_published(&self, note_id: i32) -> RepositoryResult<Option<Note>>;
}
