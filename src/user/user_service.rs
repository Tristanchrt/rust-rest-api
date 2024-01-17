use axum::async_trait;

use super::{
    models_users::{NewUser, Users},
    use_repository::{ConcreteUserRepository, UserRepository},
};

use diesel::{
    prelude::*,
    result::Error::{self, NotFound},
};

#[async_trait]
pub trait UserService<T> {
    async fn insert(&self, user: NewUser) -> Result<T, Error>;
    async fn fetch_all(&self) -> Result<Vec<T>, Error>;
    async fn get(&self, id: i32) -> Result<Vec<T>, Error>;
    async fn delete(&self, id: i32) -> Result<&str, Error>;
}
pub struct ConcreteUserService {
    repository: ConcreteUserRepository,
}

#[async_trait]
impl UserService<Users> for ConcreteUserService {
    async fn insert(&self, user: NewUser) -> Result<Users, Error> {
        return self.repository.insert(user).await;
    }

    async fn fetch_all(&self) -> Result<Vec<Users>, Error> {
        return self.repository.fetch_all().await;
    }

    async fn get(&self, id: i32) -> Result<Vec<Users>, Error> {
        return self.repository.get(id).await;
    }

    async fn delete(&self, id: i32) -> Result<&str, Error> {
        return self.repository.delete(id).await;
    }
}

impl ConcreteUserService {
    pub fn new() -> Self {
        ConcreteUserService {
            repository: ConcreteUserRepository::new(),
        }
    }
}
