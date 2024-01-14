use axum::async_trait;
use deadpool_diesel::{
    postgres::{Manager, Object},
    Pool,
};
use pwhash::bcrypt;

use crate::libs::{internal_error, pool_creation};
use diesel::{prelude::*, result::Error};

use super::{
    models_users::{NewUser, Users},
    schema::users,
};

// impl DieselRepository {
//     pub async fn new_pool(url_db: &str) -> Result<Obj, ()> {
//         let tmp = pool_creation().get().await.map_err(internal_error)?;
//         match tmp {
//             Ok(value) => Ok(Self {
//                 db_pool: Some(value),
//             }),
//             Err(err) => Err(()),
//         }
//     }
// }

#[async_trait]
pub trait UserRepository<T> {
    async fn insert(&self, user: NewUser) -> Result<T, Error>;
    async fn fetch_all(&self) -> Result<Vec<T>, Error>;
    async fn get(&self, id: String) -> Result<T, Error>;
    async fn delete(&self, id: String) -> Result<&str, Error>;
}

pub struct ConcreteUserRepository {
    db_pool: Pool<Manager>, // Assuming Pool and Manager types are defined
}

#[async_trait]
impl UserRepository<Users> for ConcreteUserRepository {
    async fn insert(&self, mut user: NewUser) -> Result<Users, Error> {
        let conn = self
            .db_pool
            .get()
            .await
            .map_err(|err| Error::DatabaseError)
            .expect("Db error");

        user.password = bcrypt::hash(user.password).expect("Error hashing password");

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
        // Implementation for fetching all users
        // This is where you would query the database for all users
        let conn = self.db_pool.get().await.map_err(internal_error).unwrap();
        todo!()
    }

    async fn get(&self, id: String) -> Result<Users, Error> {
        // Implementation for fetching a user by ID
        // This is where you would query the database for a specific user
        let conn = self.db_pool.get().await.map_err(internal_error).unwrap();
        todo!()
    }

    async fn delete(&self, id: String) -> Result<&str, Error> {
        // Implementation for deleting a user by ID
        // This is where you would delete the user from the database
        let conn = self.db_pool.get().await.map_err(internal_error).unwrap();
        todo!()
    }
}

impl ConcreteUserRepository {
    pub fn new() -> Self {
        ConcreteUserRepository {
            db_pool: pool_creation(), // assuming pool_creation is defined
        }
    }
}
