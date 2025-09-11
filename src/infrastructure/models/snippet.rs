use crate::domain::models::snippet::{CreateSnippet, Snippet};
use crate::infrastructure::schema::snippets;
use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct SnippetDiesel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = snippets)]
pub struct CreateSnippetDiesel {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub published: bool,
}

impl From<CreateSnippet> for CreateSnippetDiesel {
    fn from(snippet: CreateSnippet) -> Self {
        CreateSnippetDiesel {
            title: snippet.title,
            slug: snippet.slug,
            description: snippet.description,
            body: snippet.body,
            published: snippet.published,
        }
    }
}

impl Into<Snippet> for SnippetDiesel {
    fn into(self) -> Snippet {
        Snippet {
            id: self.id,
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            published: self.published,
            tags: Vec::new(), // 将在repository层填充
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
