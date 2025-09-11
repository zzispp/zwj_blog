use actix_web::{web, HttpResponse};

use crate::api::dto::{
    response::ApiResponse,
    snippet::{CreateSnippetDTO, ListSnippetsDTO, UpdateSnippetDTO},
};
use crate::domain::{error::ApiError, services::snippet::SnippetService};

pub async fn create_snippet_handler(
    body: web::Json<CreateSnippetDTO>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let create_snippet = body.into_inner().into();
    match snippet_service.create(create_snippet).await {
        Ok(snippet) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippet))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn list_snippets_handler(
    body: web::Json<ListSnippetsDTO>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let params = body.into_inner().into();
    match snippet_service.list(params).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_snippet_handler(
    path: web::Path<i32>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let snippet_id = path.into_inner();
    match snippet_service.get(snippet_id).await {
        Ok(Some(snippet)) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippet))),
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Snippet not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_snippet_by_slug_handler(
    path: web::Path<String>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let slug = path.into_inner();
    match snippet_service.get_by_slug(&slug).await {
        Ok(Some(snippet)) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippet))),
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Snippet not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_published_snippets_handler(
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    match snippet_service.get_published().await {
        Ok(snippets) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippets))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn update_snippet_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateSnippetDTO>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let snippet_id = path.into_inner();
    let update_snippet = body.into_inner().into();
    match snippet_service.update(snippet_id, update_snippet).await {
        Ok(Some(snippet)) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippet))),
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Snippet not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn delete_snippet_handler(
    path: web::Path<i32>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let snippet_id = path.into_inner();
    match snippet_service.delete(snippet_id).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"success": true}))))
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Snippet not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn snippet_exists_handler(
    path: web::Path<i32>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let snippet_id = path.into_inner();
    match snippet_service.exists(snippet_id).await {
        Ok(exists) => Ok(
            HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"exists": exists})))
        ),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn toggle_snippet_published_handler(
    path: web::Path<i32>,
    snippet_service: web::Data<dyn SnippetService>,
) -> Result<HttpResponse, ApiError> {
    let snippet_id = path.into_inner();
    match snippet_service.toggle_published(snippet_id).await {
        Ok(Some(snippet)) => Ok(HttpResponse::Ok().json(ApiResponse::success(snippet))),
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Snippet not found")))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}
