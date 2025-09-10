use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{error::CommonError, repositories::user::UserRepository, services::user::UserService};

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_nonce(&self, address: String) -> Result<String, CommonError> {
        //这里要检查Redis 如果存在，就返回，不存在就生成并存储
        
        self.repository
            .get_nonce()
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
