use actix_web::{web, HttpResponse};

use crate::api::dto::{
    note::{CreateNoteDTO, ListNotesDTO, UpdateNoteDTO},
    response::ApiResponse,
};
use crate::domain::{error::ApiError, services::note::NoteService};

pub async fn create_note_handler(
    body: web::Json<CreateNoteDTO>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let create_note = body.into_inner().into();
    match note_service.create(create_note).await {
        Ok(note) => Ok(HttpResponse::Ok().json(ApiResponse::success(note))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn list_notes_handler(
    body: web::Json<ListNotesDTO>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let params = body.into_inner().into();
    match note_service.list(params).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_note_handler(
    path: web::Path<i32>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let note_id = path.into_inner();
    match note_service.get(note_id).await {
        Ok(Some(note)) => Ok(HttpResponse::Ok().json(ApiResponse::success(note))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Note not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn get_all_notes_handler(
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    match note_service.get_all().await {
        Ok(notes) => Ok(HttpResponse::Ok().json(ApiResponse::success(notes))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn update_note_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateNoteDTO>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let note_id = path.into_inner();
    let update_note = body.into_inner().into();
    match note_service.update(note_id, update_note).await {
        Ok(Some(note)) => Ok(HttpResponse::Ok().json(ApiResponse::success(note))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Note not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn delete_note_handler(
    path: web::Path<i32>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let note_id = path.into_inner();
    match note_service.delete(note_id).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"success": true}))))
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Note not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn note_exists_handler(
    path: web::Path<i32>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let note_id = path.into_inner();
    match note_service.exists(note_id).await {
        Ok(exists) => Ok(
            HttpResponse::Ok().json(ApiResponse::success(serde_json::json!({"exists": exists})))
        ),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}

pub async fn toggle_note_published_handler(
    path: web::Path<i32>,
    note_service: web::Data<dyn NoteService>,
) -> Result<HttpResponse, ApiError> {
    let note_id = path.into_inner();
    match note_service.toggle_published(note_id).await {
        Ok(Some(note)) => Ok(HttpResponse::Ok().json(ApiResponse::success(note))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Note not found"))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string())))
        }
    }
}
