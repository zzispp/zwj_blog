use actix_web::web;

use crate::{
    api::dto::{
        response::ApiResponse,
        tag::{CreateTagDTO, UpdateTagDTO},
    },
    domain::{
        error::ApiError,
        models::tag::{Tag, TagType},
        repositories::{repository::ResultPaging, tag::TagQueryParams},
        services::tag::TagService,
    },
};

pub async fn create_tag_handler(
    tag_service: web::Data<dyn TagService>,
    post_data: web::Json<CreateTagDTO>,
) -> Result<ApiResponse<()>, ApiError> {
    tag_service.create(post_data.into_inner().into()).await?;
    Ok(ApiResponse::success(()))
}

pub async fn list_tags_handler(
    tag_service: web::Data<dyn TagService>,
    post_data: web::Json<TagQueryParams>,
) -> Result<ApiResponse<ResultPaging<Tag>>, ApiError> {
    let tags = tag_service.list(post_data.into_inner()).await?;
    Ok(ApiResponse::success(tags.into()))
}

pub async fn get_tag_handler(
    tag_service: web::Data<dyn TagService>,
    path: web::Path<i32>,
) -> Result<ApiResponse<Option<Tag>>, ApiError> {
    let tag_id = path.into_inner();
    let tag = tag_service.get(tag_id).await?;
    Ok(ApiResponse::success(tag))
}

pub async fn update_tag_handler(
    tag_service: web::Data<dyn TagService>,
    path: web::Path<i32>,
    post_data: web::Json<UpdateTagDTO>,
) -> Result<ApiResponse<Option<Tag>>, ApiError> {
    let tag_id = path.into_inner();
    let updated_tag = tag_service
        .update(tag_id, post_data.into_inner().into())
        .await?;
    Ok(ApiResponse::success(updated_tag))
}

pub async fn delete_tag_handler(
    tag_service: web::Data<dyn TagService>,
    path: web::Path<i32>,
) -> Result<ApiResponse<bool>, ApiError> {
    let tag_id = path.into_inner();
    let success = tag_service.delete(tag_id).await?;
    Ok(ApiResponse::success(success))
}

pub async fn tag_exists_handler(
    tag_service: web::Data<dyn TagService>,
    path: web::Path<i32>,
) -> Result<ApiResponse<bool>, ApiError> {
    let tag_id = path.into_inner();
    let exists = tag_service.exists(tag_id).await?;
    Ok(ApiResponse::success(exists))
}

#[derive(serde::Deserialize)]
pub struct GetAllTagsQuery {
    #[serde(rename = "type")]
    pub tag_type: Option<String>,
}

pub async fn get_all_tags_handler(
    tag_service: web::Data<dyn TagService>,
    query: web::Query<GetAllTagsQuery>,
) -> Result<ApiResponse<Vec<Tag>>, ApiError> {
    let tag_type = query.tag_type.as_ref().and_then(|t| match t.as_str() {
        "ALL" => Some(TagType::All),
        "BLOG" => Some(TagType::Blog),
        "NOTE" => Some(TagType::Note),
        "SNIPPET" => Some(TagType::Snippet),
        _ => None,
    });

    let tags = tag_service.get_all(tag_type).await?;
    Ok(ApiResponse::success(tags))
}
