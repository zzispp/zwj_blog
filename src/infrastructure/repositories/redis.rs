use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

use crate::{
    domain::error::RepositoryError,
    domain::repositories::{redis::RedisRepository, repository::RepositoryResult},
};

pub struct RedisClientRepository {
    client: Arc<Client>,
}

impl RedisClientRepository {
    pub fn new(client: Arc<Client>) -> Self {
        RedisClientRepository { client }
    }
}

#[async_trait]
impl<T> RedisRepository<T> for RedisClientRepository
where
    T: Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static,
{
    async fn set(&self, key: &str, value: T) -> RepositoryResult<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| RepositoryError::SerializationError(e.to_string()))?;

        let _: () = conn
            .set(key, serialized_value)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }


    async fn set_with_ttl(&self, key: &str, value: T, ttl: Duration) -> RepositoryResult<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| RepositoryError::SerializationError(e.to_string()))?;

        let _: () = conn
            .set_ex(key, serialized_value, ttl.as_secs())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get(&self, key: &str) -> RepositoryResult<Option<T>> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let result: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match result {
            Some(serialized_value) => {
                let value = serde_json::from_str(serialized_value.as_str())
                    .map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, key: &str) -> RepositoryResult<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let _: () = conn
            .del(key)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn exists(&self, key: &str) -> RepositoryResult<bool> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let exists: bool = conn
            .exists(key)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }

    async fn set_ex(&self, key: &str, value: T, seconds: u64) -> RepositoryResult<()> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| RepositoryError::SerializationError(e.to_string()))?;

        let _: () = conn
            .set_ex(key, serialized_value, seconds)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
