use crate::domain::models::blog::{Blog, CreateBlog};
use crate::infrastructure::schema::blogs;
use chrono::{DateTime, Utc};
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct BlogDiesel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = blogs)]
pub struct CreateBlogDiesel {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
    pub cover: Option<String>,
    pub author: Option<String>,
    pub published: bool,
}

impl From<CreateBlog> for CreateBlogDiesel {
    fn from(blog: CreateBlog) -> Self {
        CreateBlogDiesel {
            title: blog.title,
            slug: blog.slug,
            description: blog.description,
            body: blog.body,
            cover: blog.cover,
            author: blog.author,
            published: blog.published,
        }
    }
}

impl Into<Blog> for BlogDiesel {
    fn into(self) -> Blog {
        Blog {
            id: self.id,
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            cover: self.cover,
            author: self.author,
            published: self.published,
            tags: Vec::new(), // 将在repository层填充
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
