use crate::domain::models::snippet::{CreateSnippet, UpdateSnippet};
use crate::domain::repositories::snippet::SnippetQueryParams;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSnippetDTO {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<CreateSnippet> for CreateSnippetDTO {
    fn into(self) -> CreateSnippet {
        CreateSnippet {
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            published: self.published.unwrap_or(false),
            tag_ids: self.tags.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateSnippetDTO {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<UpdateSnippet> for UpdateSnippetDTO {
    fn into(self) -> UpdateSnippet {
        UpdateSnippet {
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            published: self.published,
            tag_ids: self.tags,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListSnippetsDTO {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<String>,
    pub order: Option<String>,
}

impl Into<SnippetQueryParams> for ListSnippetsDTO {
    fn into(self) -> SnippetQueryParams {
        SnippetQueryParams {
            limit: self.limit,
            offset: self.offset,
            title: self.title,
            slug: self.slug,
            published: self.published,
            tags: self.tags,
            order_by: self.order_by,
            order: self.order,
        }
    }
}
