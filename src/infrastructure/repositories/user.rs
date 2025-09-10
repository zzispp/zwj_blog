use async_trait::async_trait;
use rand::{distributions::Alphanumeric, prelude::Distribution};
use std::sync::Arc;

use crate::{
    domain::repositories::{repository::RepositoryResult, user::UserRepository},
    infrastructure::databases::postgresql::DBConn,
};

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>,
}

impl UserDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {
    async fn get_nonce(&self) -> RepositoryResult<String> {
        let mut rng = rand::thread_rng();

        Ok(Alphanumeric
            .sample_iter(&mut rng)
            .take(32)
            .map(char::from)
            .collect::<String>())
    }
}
