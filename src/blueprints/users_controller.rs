use crate::users;
use crate::{
    libs::internal_error,
    user::models_users::{NewUser, Users},
};
use axum::extract::Path;
use axum::{
    extract::Query,
    extract::{MatchedPath, Request, State},
    http::StatusCode,
    response::Json,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use diesel::prelude::*;
use pwhash::bcrypt;

#[debug_handler]
pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Users>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users::table.select(Users::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[debug_handler]
pub async fn get_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path((user_id)): Path<(i32)>,
) -> Result<Json<Vec<Users>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            users::table
                .find(user_id)
                .select(Users::as_select())
                .load(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[debug_handler]
pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(mut new_user): Json<NewUser>,
) -> Result<Json<Users>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    new_user.password = bcrypt::hash(new_user.password).expect("Error hashing password");
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(Users::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
