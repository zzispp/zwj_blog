use async_trait::async_trait;

use crate::domain::error::CommonError;


#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn get_nonce(&self, address: String) -> Result<String, CommonError>;
    async fn verify_signature(&self, address: String, signature: String) -> Result<(), CommonError>;
}