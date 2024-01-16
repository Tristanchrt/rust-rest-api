use axum::http::StatusCode;
use axum::Error;
use deadpool_diesel::{postgres::Manager, Pool};
use dotenv::dotenv;
use std::env;

pub fn pool_creation() -> Pool<Manager> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DB_HOST must be set");
    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    return deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Error pool connection");
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
