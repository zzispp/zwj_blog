use crate::api::dto::response::ApiResponse;
use crate::api::dto::todo::{CreateTodoDTO, TodoDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;
use crate::domain::services::todo::TodoService;
use actix_web::{web, Result};

pub async fn create_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    post_data: web::Json<CreateTodoDTO>,
) -> Result<ApiResponse<TodoDTO>, ApiError> {
    let todo = todo_service.create(post_data.into_inner().into()).await?;
    Ok(ApiResponse::success(todo.into()))
}

pub async fn list_todos_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Query<TodoQueryParams>,
) -> Result<ApiResponse<ResultPaging<TodoDTO>>, ApiError> {
    let selection = todo_service.list(params.into_inner()).await?;
    Ok(ApiResponse::success(selection.into()))
}

pub async fn get_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Path<i32>,
) -> Result<ApiResponse<TodoDTO>, ApiError> {
    let todo = todo_service.get(params.into_inner()).await?;
    Ok(ApiResponse::success(todo.into()))
}

pub async fn delete_todo_handler(
    todo_service: web::Data<dyn TodoService>,
    params: web::Path<i32>,
) -> Result<ApiResponse<()>, ApiError> {
    todo_service.delete(params.into_inner()).await?;
    Ok(ApiResponse::success(()))
}
