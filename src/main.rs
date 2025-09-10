use zwj_blog::{
    config::AppConfig, container::Container, create_app::create_app, logging,
};
use actix_web::HttpServer;
use std::sync::Arc;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::new().expect("Failed to load configuration");

    logging::init_tracing(&config);

    tracing::info!("Starting {} server", config.app.name);
    tracing::info!(
        "Server will listen on {}:{} with {} workers",
        config.app.host,
        config.app.port,
        config.app.workers
    );

    let container = Arc::new(Container::new(&config));
    let server = HttpServer::new(move || create_app(container.clone()))
        .workers(config.app.workers)
        .bind((config.app.host.clone(), config.app.port))?;

    tracing::info!("Server started successfully");
    server.run().await
}
