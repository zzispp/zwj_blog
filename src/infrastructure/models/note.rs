use crate::domain::models::note::{CreateNote, Note};
use crate::infrastructure::schema::notes;
use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct NoteDiesel {
    pub id: i32,
    pub body: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct CreateNoteDiesel {
    pub body: String,
    pub published: bool,
}

impl From<CreateNote> for CreateNoteDiesel {
    fn from(note: CreateNote) -> Self {
        CreateNoteDiesel {
            body: note.body,
            published: note.published,
        }
    }
}

impl Into<Note> for NoteDiesel {
    fn into(self) -> Note {
        Note {
            id: self.id,
            body: self.body,
            published: self.published,
            tags: Vec::new(), // 将在repository层填充
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
