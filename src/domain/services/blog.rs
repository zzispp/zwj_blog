use async_trait::async_trait;

use crate::domain::{
    error::CommonError,
    models::blog::{Blog, CreateBlog, UpdateBlog},
    repositories::{blog::BlogQueryParams, repository::ResultPaging},
};

#[async_trait]
pub trait BlogService: 'static + Sync + Send {
    async fn create(&self, blog: CreateBlog) -> Result<Blog, CommonError>;
    async fn list(&self, params: BlogQueryParams) -> Result<ResultPaging<Blog>, CommonError>;
    async fn get(&self, blog_id: i32) -> Result<Option<Blog>, CommonError>;
    async fn get_by_slug(&self, slug: &str) -> Result<Option<Blog>, CommonError>;
    async fn get_published(&self) -> Result<Vec<Blog>, CommonError>;
    async fn get_published_by_slug(&self, slug: &str) -> Result<Option<Blog>, CommonError>;
    async fn update(
        &self,
        blog_id: i32,
        update_blog: UpdateBlog,
    ) -> Result<Option<Blog>, CommonError>;
    async fn delete(&self, blog_id: i32) -> Result<bool, CommonError>;
    async fn exists(&self, blog_id: i32) -> Result<bool, CommonError>;
    async fn toggle_published(&self, blog_id: i32) -> Result<Option<Blog>, CommonError>;
}
