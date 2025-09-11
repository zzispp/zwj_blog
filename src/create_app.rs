use crate::api::controllers::blog_handler::{
    blog_exists_handler, create_blog_handler, delete_blog_handler, get_blog_by_slug_handler,
    get_blog_handler, get_published_blog_by_slug_handler, get_published_blogs_handler,
    list_blogs_handler, toggle_blog_published_handler, update_blog_handler,
};
use crate::api::controllers::file_handler::upload_file_handler;
use crate::api::controllers::note_handler::{
    create_note_handler, delete_note_handler, get_all_notes_handler, get_note_handler,
    list_notes_handler, note_exists_handler, toggle_note_published_handler, update_note_handler,
};
use crate::api::controllers::snippet_handler::{
    create_snippet_handler, delete_snippet_handler, get_published_snippets_handler,
    get_snippet_by_slug_handler, get_snippet_handler, list_snippets_handler,
    snippet_exists_handler, toggle_snippet_published_handler, update_snippet_handler,
};
use crate::api::controllers::tag_handler::{
    create_tag_handler, delete_tag_handler, get_all_tags_handler, get_tag_handler,
    list_tags_handler, tag_exists_handler, update_tag_handler,
};
use crate::api::controllers::todo_handler::{
    create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler,
};
use crate::api::controllers::user_handler::{get_nonce_handler, verify_signature_handler};
use crate::api::middleware::JwtMiddleware;
use crate::api::middleware::ServiceContextMaintenanceCheck;
use crate::container::Container;
use actix_files as fs;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::{web, App};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub fn create_app(
    container: Arc<Container>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let todo_service = container.todo_service.clone();
    let user_service = container.user_service.clone();
    let file_service = container.file_service.clone();
    let tag_service = container.tag_service.clone();
    let blog_service = container.blog_service.clone();
    let note_service = container.note_service.clone();
    let snippet_service = container.snippet_service.clone();
    let service_context_service = container.service_context_service.clone();

    App::new()
        .app_data(web::Data::from(todo_service.clone()))
        .app_data(web::Data::from(user_service.clone()))
        .app_data(web::Data::from(file_service.clone()))
        .app_data(web::Data::from(tag_service.clone()))
        .app_data(web::Data::from(blog_service.clone()))
        .app_data(web::Data::from(note_service.clone()))
        .app_data(web::Data::from(snippet_service.clone()))
        .app_data(web::Data::from(service_context_service.clone()))
        .wrap(TracingLogger::default())
        .wrap(ServiceContextMaintenanceCheck)
        .wrap(JwtMiddleware)
        .service(
            web::scope("/api")
                .service(
                    web::scope("/todos")
                        .route("", web::post().to(create_todo_handler))
                        .route("", web::get().to(list_todos_handler))
                        .route("/{id}", web::get().to(get_todo_handler))
                        .route("/{id}", web::delete().to(delete_todo_handler)),
                )
                .service(
                    web::scope("/users")
                        .route("/nonce", web::post().to(get_nonce_handler))
                        .route("/verify", web::post().to(verify_signature_handler)),
                )
                .service(web::scope("/files").route("/upload", web::post().to(upload_file_handler)))
                .service(
                    web::scope("/tags")
                        .route("/create", web::post().to(create_tag_handler))
                        .route("/list", web::post().to(list_tags_handler))
                        .route("/all", web::get().to(get_all_tags_handler))
                        .route("/{id}", web::get().to(get_tag_handler))
                        .route("/{id}", web::put().to(update_tag_handler))
                        .route("/{id}", web::delete().to(delete_tag_handler))
                        .route("/{id}/exists", web::get().to(tag_exists_handler)),
                )
                .service(
                    web::scope("/blogs")
                        .route("/create", web::post().to(create_blog_handler))
                        .route("/list", web::post().to(list_blogs_handler))
                        .route("/published", web::get().to(get_published_blogs_handler))
                        .route("/slug/{slug}", web::get().to(get_blog_by_slug_handler))
                        .route(
                            "/published/slug/{slug}",
                            web::get().to(get_published_blog_by_slug_handler),
                        )
                        .route("/{id}", web::get().to(get_blog_handler))
                        .route("/{id}", web::put().to(update_blog_handler))
                        .route("/{id}", web::delete().to(delete_blog_handler))
                        .route("/{id}/exists", web::get().to(blog_exists_handler))
                        .route(
                            "/{id}/toggle-publish",
                            web::patch().to(toggle_blog_published_handler),
                        ),
                )
                .service(
                    web::scope("/notes")
                        .route("/create", web::post().to(create_note_handler))
                        .route("/list", web::post().to(list_notes_handler))
                        .route("/all", web::get().to(get_all_notes_handler))
                        .route("/{id}", web::get().to(get_note_handler))
                        .route("/{id}", web::put().to(update_note_handler))
                        .route("/{id}", web::delete().to(delete_note_handler))
                        .route("/{id}/exists", web::get().to(note_exists_handler))
                        .route(
                            "/{id}/toggle-publish",
                            web::patch().to(toggle_note_published_handler),
                        ),
                )
                .service(
                    web::scope("/snippets")
                        .route("/create", web::post().to(create_snippet_handler))
                        .route("/list", web::post().to(list_snippets_handler))
                        .route("/published", web::get().to(get_published_snippets_handler))
                        .route("/slug/{slug}", web::get().to(get_snippet_by_slug_handler))
                        .route("/{id}", web::get().to(get_snippet_handler))
                        .route("/{id}", web::put().to(update_snippet_handler))
                        .route("/{id}", web::delete().to(delete_snippet_handler))
                        .route("/{id}/exists", web::get().to(snippet_exists_handler))
                        .route(
                            "/{id}/toggle-publish",
                            web::patch().to(toggle_snippet_published_handler),
                        ),
                ),
        )
        // 静态文件服务器 - 提供上传的文件访问
        .service(fs::Files::new("/static", "uploads")) //uploads是文件的映射路径
}
