use async_trait::async_trait;

use crate::domain::repositories::repository::RepositoryResult;


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_nonce(&self) -> RepositoryResult<String>;
}
