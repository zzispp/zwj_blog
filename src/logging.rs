use crate::config::AppConfig;

pub fn init_tracing(config: &AppConfig) {
    use std::str::FromStr;
    let level = tracing::Level::from_str(&config.logging.level).unwrap_or(tracing::Level::INFO);

    match config.logging.format.as_str() {
        "json" => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(level)
                .init();
        }
        _ => {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(level)
                .init();
        }
    }
}
