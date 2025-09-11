// Note service trait - placeholder
use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::note::{CreateNote, Note, UpdateNote},
    repositories::{note::NoteQueryParams, repository::ResultPaging},
};

#[async_trait]
pub trait NoteService: 'static + Sync + Send {
    async fn create(&self, note: CreateNote) -> Result<Note, CommonError>;
    async fn list(&self, params: NoteQueryParams) -> Result<ResultPaging<Note>, CommonError>;
    async fn get(&self, note_id: i32) -> Result<Option<Note>, CommonError>;
    async fn get_all(&self) -> Result<Vec<Note>, CommonError>;
    async fn update(
        &self,
        note_id: i32,
        update_note: UpdateNote,
    ) -> Result<Option<Note>, CommonError>;
    async fn delete(&self, note_id: i32) -> Result<bool, CommonError>;
    async fn exists(&self, note_id: i32) -> Result<bool, CommonError>;
    async fn toggle_published(&self, note_id: i32) -> Result<Option<Note>, CommonError>;
}
