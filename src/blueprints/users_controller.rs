use crate::users;
use crate::{
    libs::internal_error,
    user::models_users::{NewUser, Users},
};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Json};
use axum_macros::debug_handler;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde_json::Value;

#[debug_handler]
pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users::table.select(Users::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    let json_response: Value = serde_json::json!({
        "status": "ok",
        "count": res.len(),
        "objects": res
    });

    Ok(Json(json_response))
}

#[debug_handler]
pub async fn get_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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

    let json_response = serde_json::json!({
        "status": "ok",
        "object": res[0]
    });

    Ok(Json(json_response))
}

#[debug_handler]
pub async fn delete_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    // Use Diesel to delete the user with the specified user_id
    let deleted_rows = conn
        .interact(move |conn| diesel::delete(users::table.find(user_id)).execute(conn))
        .await
        .map_err(internal_error)?;

    if deleted_rows == Ok(1) {
        let json_response = serde_json::json!({
            "status": "ok",
            "message": "User deleted successfully",
        });
        Ok(Json(json_response))
    } else {
        // User with the specified ID not found
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}

#[debug_handler]
pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(mut new_user): Json<NewUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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

    let json_response = serde_json::json!({
        "status": "ok",
        "object": res
    });

    Ok(Json(json_response))
}
