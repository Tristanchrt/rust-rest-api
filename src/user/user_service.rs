use axum::async_trait;

use super::models_users::{NewUser, Users};

#[async_trait]
pub trait UserService<T> {
    async fn insert(&self, user: NewUser) -> Result<T, String>;
    async fn fetch_all(&self) -> Result<Vec<T>, String>;
    async fn get(&self, id: String) -> Result<T, String>;
    async fn delete(&self, id: String) -> Result<String, String>;
}
pub struct ConcreteUserService;

#[async_trait]
impl UserService<Users> for ConcreteUserService {
    async fn insert(&self, user: NewUser) -> Result<Users, String> {
        // Implementation for insert
        // This is where you would actually insert the user into the database
        todo!()
    }

    async fn fetch_all(&self) -> Result<Vec<Users>, String> {
        // Implementation for fetching all users
        // This is where you would query the database for all users
        todo!()
    }

    async fn get(&self, id: String) -> Result<Users, String> {
        // Implementation for fetching a user by ID
        // This is where you would query the database for a specific user
        todo!()
    }

    async fn delete(&self, id: String) -> Result<String, String> {
        // Implementation for deleting a user by ID
        // This is where you would delete the user from the database
        todo!()
    }
}

impl ConcreteUserService {
    pub fn new() -> Self {
        ConcreteUserService
    }
}
