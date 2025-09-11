use crate::domain::models::tag::Tag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Note {
    pub id: i32,
    pub body: String,
    pub published: bool,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CreateNote {
    pub body: String,
    pub published: bool,
    pub tag_ids: Vec<i32>,
}

#[derive(Clone)]
pub struct UpdateNote {
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tag_ids: Option<Vec<i32>>,
}
