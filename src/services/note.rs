use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    error::CommonError,
    models::note::{CreateNote, Note, UpdateNote},
    repositories::{
        note::{NoteQueryParams, NoteRepository},
        repository::ResultPaging,
    },
    services::note::NoteService,
};

#[derive(Clone)]
pub struct NoteServiceImpl {
    pub repository: Arc<dyn NoteRepository>,
}

impl NoteServiceImpl {
    pub fn new(repository: Arc<dyn NoteRepository>) -> Self {
        NoteServiceImpl { repository }
    }
}

#[async_trait]
impl NoteService for NoteServiceImpl {
    async fn create(&self, note: CreateNote) -> Result<Note, CommonError> {
        self.repository
            .create(&note)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: NoteQueryParams) -> Result<ResultPaging<Note>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, note_id: i32) -> Result<Option<Note>, CommonError> {
        self.repository
            .get(note_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_all(&self) -> Result<Vec<Note>, CommonError> {
        self.repository
            .get_all()
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn update(
        &self,
        note_id: i32,
        update_note: UpdateNote,
    ) -> Result<Option<Note>, CommonError> {
        self.repository
            .update(note_id, &update_note)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, note_id: i32) -> Result<bool, CommonError> {
        self.repository
            .delete(note_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn exists(&self, note_id: i32) -> Result<bool, CommonError> {
        self.repository
            .exists(note_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn toggle_published(&self, note_id: i32) -> Result<Option<Note>, CommonError> {
        self.repository
            .toggle_published(note_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
