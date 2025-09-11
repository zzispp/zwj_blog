use actix_web::{web, HttpResponse};

use crate::api::dto::{
    blog::{CreateBlogDTO, ListBlogsDTO, UpdateBlogDTO},
    response::ApiResponse,
};
use crate::domain::{error::ApiError, services::blog::BlogService};

pub async fn create_blog_handler(
    body: web::Json<CreateBlogDTO>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let create_blog = body.into_inner().into();
    match blog_service.create(create_blog).await {
        Ok(blog) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn list_blogs_handler(
    body: web::Json<ListBlogsDTO>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let params = body.into_inner().into();
    match blog_service.list(params).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_blog_handler(
    path: web::Path<i32>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let blog_id = path.into_inner();
    match blog_service.get(blog_id).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Blog not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_blog_by_slug_handler(
    path: web::Path<String>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let slug = path.into_inner();
    match blog_service.get_by_slug(&slug).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Blog not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_published_blogs_handler(
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    match blog_service.get_published().await {
        Ok(blogs) => Ok(HttpResponse::Ok().json(ApiResponse::success(blogs))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_published_blog_by_slug_handler(
    path: web::Path<String>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let slug = path.into_inner();
    match blog_service.get_published_by_slug(&slug).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Published blog not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn update_blog_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateBlogDTO>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let blog_id = path.into_inner();
    let update_blog = body.into_inner().into();
    match blog_service.update(blog_id, update_blog).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Blog not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn delete_blog_handler(
    path: web::Path<i32>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let blog_id = path.into_inner();
    match blog_service.delete(blog_id).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"success": true}))))
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Blog not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn blog_exists_handler(
    path: web::Path<i32>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let blog_id = path.into_inner();
    match blog_service.exists(blog_id).await {
        Ok(exists) => Ok(
            HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"exists": exists})))
        ),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn toggle_blog_published_handler(
    path: web::Path<i32>,
    blog_service: web::Data<dyn BlogService>,
) -> Result<HttpResponse, ApiError> {
    let blog_id = path.into_inner();
    match blog_service.toggle_published(blog_id).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(ApiResponse::success(blog))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Blog not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}
