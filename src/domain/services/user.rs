use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::error::CommonError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub address: String,
}

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn get_nonce(&self, address: String) -> Result<String, CommonError>;
    async fn verify_signature(
        &self,
        address: String,
        signature: String,
    ) -> Result<AuthResponse, CommonError>;
}
