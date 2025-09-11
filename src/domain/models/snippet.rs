use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::models::tag::Tag;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Snippet {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub published: bool,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CreateSnippet {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub published: bool,
    pub tag_ids: Vec<i32>,
}

#[derive(Clone)]
pub struct UpdateSnippet {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tag_ids: Option<Vec<i32>>,
}
