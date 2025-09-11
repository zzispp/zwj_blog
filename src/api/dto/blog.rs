use crate::domain::models::blog::{CreateBlog, UpdateBlog};
use crate::domain::repositories::blog::BlogQueryParams;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateBlogDTO {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<CreateBlog> for CreateBlogDTO {
    fn into(self) -> CreateBlog {
        CreateBlog {
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            cover: self.cover,
            author: self.author,
            published: self.published.unwrap_or(false),
            tag_ids: self.tags.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBlogDTO {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}

impl Into<UpdateBlog> for UpdateBlogDTO {
    fn into(self) -> UpdateBlog {
        UpdateBlog {
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            cover: self.cover,
            author: self.author,
            published: self.published,
            tag_ids: self.tags,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ListBlogsDTO {
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

impl Into<BlogQueryParams> for ListBlogsDTO {
    fn into(self) -> BlogQueryParams {
        BlogQueryParams {
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
