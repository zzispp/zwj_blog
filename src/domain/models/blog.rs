use crate::domain::models::tag::Tag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: bool,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CreateBlog {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: bool,
    pub tag_ids: Vec<i32>,
}

#[derive(Clone)]
pub struct UpdateBlog {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
    pub tag_ids: Option<Vec<i32>>,
}
