use crate::config::AppConfig;
use crate::domain::repositories::file::FileRepository;
use crate::domain::repositories::redis::RedisRepository;
use crate::domain::repositories::tag::TagRepository;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::file::FileService;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::tag::TagService;
use crate::domain::services::todo::TodoService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::file::FileDieselRepository;
use crate::infrastructure::repositories::redis::RedisClientRepository;
use crate::infrastructure::repositories::tag::TagDieselRepository;
use crate::infrastructure::repositories::todo::TodoDieselRepository;
use crate::infrastructure::repositories::user::UserDieselRepository;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::file::FileServiceImpl;
use crate::services::tag::TagServiceImpl;
use crate::services::todo::TodoServiceImpl;
use crate::services::user::UserServiceImpl;
use redis::Client;
use std::sync::Arc;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>,
    pub file_service: Arc<dyn FileService>,
    pub tag_service: Arc<dyn TagService>,
    pub service_context_service: Arc<dyn ServiceContextService>,
}

impl Container {
    pub fn new(config: &AppConfig) -> Self {
        let pool = Arc::new(db_pool(config));
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(pool.clone()));
        let todo_service = Arc::new(TodoServiceImpl {
            repository: todo_repository,
        });
        let file_repository: Arc<dyn FileRepository> =
            Arc::new(FileDieselRepository::new(pool.clone()));
                
        let file_service = Arc::new(FileServiceImpl::new(file_repository));

        let tag_repository: Arc<dyn TagRepository> =
            Arc::new(TagDieselRepository::new(pool.clone()));
        let tag_service = Arc::new(TagServiceImpl::new(tag_repository));

        let redis_url = format!("redis://{}:{}@{}:{}/{}", 
            config.redis.username, 
            config.redis.password, 
            config.redis.host, 
            config.redis.port,
            config.redis.db);
        let redis_client = Arc::new(
            Client::open(redis_url.as_str()).expect("Failed to create Redis client"),
        );
        let redis_repository: Arc<dyn RedisRepository<String>> =
            Arc::new(RedisClientRepository::new(redis_client));
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserDieselRepository::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository,
            redis_repository,
        });
        let service_context_service = Arc::new(ServiceContextServiceImpl::new(pool.clone()));
        Container {
            todo_service,
            user_service,
            file_service,
            tag_service,
            service_context_service,
        }
    }
}
