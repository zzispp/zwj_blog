use std::{str::FromStr, sync::Arc, time::Duration};

use async_trait::async_trait;
use jwt_simple::prelude::*;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use tokio::fs;

use crate::{
    constants::nonce::REDIS_NONCE_KEY,
    domain::{
        error::CommonError,
        repositories::{redis::RedisRepository, user::UserRepository},
        services::user::{AuthResponse, UserService},
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

    /// 获取或创建JWT密钥
    async fn get_or_create_jwt_key(&self) -> Result<HS256Key, CommonError> {
        let key_file_path = "./jwt_key.bin";

        // 检查密钥文件是否存在
        if fs::metadata(key_file_path).await.is_ok() {
            // 密钥文件存在，读取并加载
            let key_bytes = fs::read(key_file_path).await.map_err(|e| {
                CommonError::from(format!("Failed to read JWT key file: {}", e).as_str())
            })?;

            let key = HS256Key::from_bytes(&key_bytes);

            Ok(key)
        } else {
            // 密钥文件不存在，生成新密钥并保存
            let key = HS256Key::generate();
            let key_bytes = key.to_bytes();

            fs::write(key_file_path, &key_bytes).await.map_err(|e| {
                CommonError::from(format!("Failed to save JWT key file: {}", e).as_str())
            })?;

            Ok(key)
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
    ) -> Result<AuthResponse, CommonError> {
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

        // 签名验证成功，清除Redis中的nonce
        self.redis_repository
            .delete(&format!("{}{}", REDIS_NONCE_KEY, &address))
            .await
            .map_err(|e| -> CommonError { e.into() })?;

        // 生成JWT token
        let key = self.get_or_create_jwt_key().await?;

        // 创建JWT claims，token有效期2小时
        let claims = Claims::create(jwt_simple::prelude::Duration::from_hours(2))
            .with_subject(address.clone())
            .with_issuer("zwj_blog");

        let token = key.authenticate(claims).map_err(|e| {
            CommonError::from(format!("Failed to create JWT token: {}", e).as_str())
        })?;

        Ok(AuthResponse { token, address })
    }
}
