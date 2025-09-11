use crate::domain::models::note::{CreateNote, UpdateNote};
use crate::domain::repositories::note::NoteQueryParams;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateNoteDTO {
    pub body: String,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<CreateNote> for CreateNoteDTO {
    fn into(self) -> CreateNote {
        CreateNote {
            body: self.body,
            published: self.published.unwrap_or(false),
            tag_ids: self.tags.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateNoteDTO {
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<UpdateNote> for UpdateNoteDTO {
    fn into(self) -> UpdateNote {
        UpdateNote {
            body: self.body,
            published: self.published,
            tag_ids: self.tags,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListNotesDTO {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<String>,
    pub order: Option<String>,
}

impl Into<NoteQueryParams> for ListNotesDTO {
    fn into(self) -> NoteQueryParams {
        NoteQueryParams {
            limit: self.limit,
            offset: self.offset,
            body: self.body,
            published: self.published,
            tags: self.tags,
            order_by: self.order_by,
            order: self.order,
        }
    }
}
