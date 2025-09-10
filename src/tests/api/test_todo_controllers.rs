#[cfg(test)]
mod test_todo_controllers {
    use zwj_blog::config::AppConfig;
    use zwj_blog::domain::models::todo::Todo;
    use zwj_blog::domain::repositories::repository::ResultPaging;
    use zwj_blog::infrastructure::databases::postgresql::db_pool;
    use zwj_blog::{container::Container, create_app::create_app};
    use actix_web::test;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serde_json;
    use serde_json::json;
    use std::env;
    use std::sync::Arc;
    use testcontainers::clients;
    use testcontainers::images::postgres;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    #[actix_web::test]
    async fn test() {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
        tracing_subscriber::fmt::init();

        let docker = clients::Cli::default();
        let postgres_node = docker.run(postgres::Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres_node.get_host_port_ipv4(5432)
        );

        // Create test configuration
        let test_config = AppConfig {
            app: zwj_blog::config::App {
                name: "test".to_string(),
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: 1,
            },
            database: zwj_blog::config::Database {
                url: connection_string.clone(),
            },
            logging: zwj_blog::config::Logging {
                level: "debug".to_string(),
                format: "pretty".to_string(),
            },
        };

        {
            let pool = Arc::new(db_pool(&test_config));
            pool.get()
                .unwrap()
                .run_pending_migrations(MIGRATIONS)
                .unwrap();
        }

        let container = Arc::new(Container::new(&test_config));

        let app = test::init_service(create_app(container)).await;
        let request_body = json!({
            "title": "test todo",
            "description": "Test description"
        });

        // Creation test
        let resp = test::TestRequest::post()
            .uri(&format!("/todos"))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let todo: Todo = test::read_body_json(resp).await;
        assert_eq!(todo.title, "test todo");
        assert_eq!(todo.description, "Test description");

        // Get all test
        let resp = test::TestRequest::get()
            .uri(&format!("/todos/{}", todo.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let retrieved_todo: Todo = test::read_body_json(resp).await;
        assert_eq!(todo.id, retrieved_todo.id);
        assert_eq!(todo.title, retrieved_todo.title);

        // Creation test
        let resp = test::TestRequest::post()
            .uri(&format!("/todos"))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());

        // Get all test
        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let todos: ResultPaging<Todo> = test::read_body_json(resp).await;
        assert_eq!(todos.items.len(), 2);

        // Delete test
        let resp = test::TestRequest::delete()
            .uri(&format!("/todos/{}", todo.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());

        // Get all test
        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let todos: ResultPaging<Todo> = test::read_body_json(resp).await;
        assert_eq!(todos.items.len(), 1);
    }
}
