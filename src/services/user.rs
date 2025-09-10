use std::{str::FromStr, sync::Arc, time::Duration};

use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::{
    constants::nonce::REDIS_NONCE_KEY,
    domain::{
        error::CommonError,
        repositories::{redis::RedisRepository, user::UserRepository},
        services::user::UserService,
    },
};

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
    pub redis_repository: Arc<dyn RedisRepository<String>>,
}

impl UserServiceImpl {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        redis_repository: Arc<dyn RedisRepository<String>>,
    ) -> Self {
        UserServiceImpl {
            repository,
            redis_repository,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_nonce(&self, address: String) -> Result<String, CommonError> {
        Pubkey::from_str(&address).map_err(|e| CommonError::from(e.to_string().as_str()))?;

        //这里要检查Redis 如果存在，就返回，不存在就生成并存储
        let nonce = self
            .redis_repository
            .get(&format!("{}{}", REDIS_NONCE_KEY, address))
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        match nonce {
            Some(existing_nonce) => return Ok(existing_nonce),
            None => {
                let nonce = self
                    .repository
                    .gen_nonce()
                    .await
                    .map_err(|e| -> CommonError { e.into() })?;
                self.redis_repository
                    .set_with_ttl(
                        &format!("{}{}", REDIS_NONCE_KEY, address),
                        nonce.clone(),
                        Duration::from_secs(60),
                    )
                    .await
                    .map_err(|e| -> CommonError { e.into() })?;
                Ok(nonce)
            }
        }
    }

    async fn verify_signature(
        &self,
        address: String,
        signature: String,
    ) -> Result<(), CommonError> {
        Pubkey::from_str(&address).map_err(|e| CommonError::from(e.to_string().as_str()))?;
        let signature = Signature::from_str(&signature)
            .map_err(|e| CommonError::from(e.to_string().as_str()))?;
        let pubkey =
            Pubkey::from_str(&address).map_err(|e| CommonError::from(e.to_string().as_str()))?;

        let nonce = self
            .redis_repository
            .get(&format!("{}{}", REDIS_NONCE_KEY, address))
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        if nonce.is_none() {
            return Err(CommonError::from("Nonce not found"));
        }
        let nonce = nonce.unwrap();
        let message = format!("nonce:{}", nonce);

        let is_valid_signature = signature.verify(&pubkey.to_bytes(), message.as_bytes());
        if !is_valid_signature {
            return Err(CommonError::from("Invalid signature"));
        }

        Ok(())
    }
}
