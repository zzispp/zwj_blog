use std::time::Duration;

use async_trait::async_trait;

use crate::domain::repositories::repository::RepositoryResult;

#[async_trait]
pub trait RedisRepository<T>: Send + Sync {
    async fn get(&self, key: &str) -> RepositoryResult<Option<T>>;
    async fn set(&self, key: &str, value: T) -> RepositoryResult<()>;
    async fn set_with_ttl(&self, key: &str, value: T, ttl: Duration) -> RepositoryResult<()>;
    async fn delete(&self, key: &str) -> RepositoryResult<()>;
    async fn exists(&self, key: &str) -> RepositoryResult<bool>;
    async fn set_ex(&self, key: &str, value: T, seconds: u64) -> RepositoryResult<()>;
}
