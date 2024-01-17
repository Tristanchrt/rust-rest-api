use axum::async_trait;
use deadpool_diesel::{
    postgres::{Manager, Object},
    Pool,
};
use pwhash::bcrypt;

use crate::libs::{internal_error, pool_creation};
use diesel::{
    prelude::*,
    result::Error::{self, NotFound},
};

use super::{
    models_users::{NewUser, Users},
    schema::users,
};
#[async_trait]
pub trait UserRepository<T> {
    async fn insert(&self, user: NewUser) -> Result<T, Error>;
    async fn fetch_all(&self) -> Result<Vec<T>, Error>;
    async fn get(&self, id: i32) -> Result<Vec<T>, Error>;
    async fn delete(&self, id: i32) -> Result<&str, Error>;
}

pub struct ConcreteUserRepository {
    db_pool: Pool<Manager>, // Assuming Pool and Manager types are defined
}

#[async_trait]
impl UserRepository<Users> for ConcreteUserRepository {
    async fn insert(&self, mut user: NewUser) -> Result<Users, Error> {
        let conn = self.db_pool.get().await.map_err(|err| Error::NotFound)?;

        user.password = bcrypt::hash(user.password).expect("Hashing password not working");

        let res: Result<Users, diesel::result::Error> = conn
            .interact(|conn| {
                diesel::insert_into(users::table)
                    .values(user)
                    .returning(Users::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::NotFound)?;

        Ok(res?)
    }

    async fn fetch_all(&self) -> Result<Vec<Users>, Error> {
        let conn = self.db_pool.get().await.map_err(|err| Error::NotFound)?;

        let res = conn
            .interact(|conn| users::table.select(Users::as_select()).load(conn))
            .await
            .map_err(|err| Error::NotFound)?;
        Ok(res?)
    }

    async fn get(&self, id: i32) -> Result<Vec<Users>, Error> {
        let conn = self.db_pool.get().await.map_err(|err| Error::NotFound)?;
        let res = conn
            .interact(move |conn| users::table.find(id).select(Users::as_select()).load(conn))
            .await
            .map_err(|err| Error::NotFound)?;

        Ok(res?)
    }

    async fn delete(&self, id: i32) -> Result<&str, Error> {
        let conn = self.db_pool.get().await.map_err(internal_error).unwrap();
        let _ = conn
            .interact(move |conn| diesel::delete(users::table.find(id)).execute(conn))
            .await
            .map_err(|err| Error::NotFound)?;
        Ok("Delete ok")
    }
}

impl ConcreteUserRepository {
    pub fn new() -> Self {
        ConcreteUserRepository {
            db_pool: pool_creation(), // assuming pool_creation is defined
        }
    }
}
