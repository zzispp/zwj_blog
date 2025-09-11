use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    error::CommonError,
    models::blog::{Blog, CreateBlog, UpdateBlog},
    repositories::{
        blog::{BlogQueryParams, BlogRepository},
        repository::ResultPaging,
    },
    services::blog::BlogService,
};

#[derive(Clone)]
pub struct BlogServiceImpl {
    pub repository: Arc<dyn BlogRepository>,
}

impl BlogServiceImpl {
    pub fn new(repository: Arc<dyn BlogRepository>) -> Self {
        BlogServiceImpl { repository }
    }
}

#[async_trait]
impl BlogService for BlogServiceImpl {
    async fn create(&self, blog: CreateBlog) -> Result<Blog, CommonError> {
        self.repository
            .create(&blog)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: BlogQueryParams) -> Result<ResultPaging<Blog>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, blog_id: i32) -> Result<Option<Blog>, CommonError> {
        self.repository
            .get(blog_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_by_slug(&self, slug: &str) -> Result<Option<Blog>, CommonError> {
        self.repository
            .get_by_slug(slug)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_published(&self) -> Result<Vec<Blog>, CommonError> {
        self.repository
            .get_published()
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get_published_by_slug(&self, slug: &str) -> Result<Option<Blog>, CommonError> {
        self.repository
            .get_published_by_slug(slug)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn update(
        &self,
        blog_id: i32,
        update_blog: UpdateBlog,
    ) -> Result<Option<Blog>, CommonError> {
        self.repository
            .update(blog_id, &update_blog)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, blog_id: i32) -> Result<bool, CommonError> {
        self.repository
            .delete(blog_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn exists(&self, blog_id: i32) -> Result<bool, CommonError> {
        self.repository
            .exists(blog_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn toggle_published(&self, blog_id: i32) -> Result<Option<Blog>, CommonError> {
        self.repository
            .toggle_published(blog_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
