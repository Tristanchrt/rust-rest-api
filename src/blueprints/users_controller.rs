use crate::user::user_service::ConcreteUserService;
use crate::users;
use crate::{
    libs::internal_error,
    user::models_users::{NewUser, Users},
    user::user_service::UserService,
};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Json};
use axum_macros::debug_handler;
use diesel::prelude::*;
use pwhash::bcrypt;
use serde_json::Value;

#[debug_handler]
pub async fn list_users() -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_service = ConcreteUserService::new();
    let res: Result<Vec<Users>, diesel::result::Error> = user_service.fetch_all().await;

    if let Err(err) = res {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        ));
    }

    let json_response = serde_json::json!({
        "status": "ok",
        "objects": res.unwrap()
    });

    Ok(Json(json_response))
}

#[debug_handler]
pub async fn get_user(Path(user_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_service = ConcreteUserService::new();
    let res: Result<Vec<Users>, diesel::result::Error> = user_service.get(user_id).await;

    if let Err(err) = res {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        ));
    }

    let json_response = serde_json::json!({
        "status": "ok",
        "object": res.unwrap()[0]
    });

    Ok(Json(json_response))
}

#[debug_handler]
pub async fn delete_user(
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_service = ConcreteUserService::new();
    let res = user_service.delete(user_id).await;

    if let Err(err) = res {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        ));
    }

    let json_response = serde_json::json!({
        "status": "ok",
        "message": "User deleted successfully",
    });

    Ok(Json(json_response))
}

#[debug_handler]
pub async fn create_user(
    Json(mut new_user): Json<NewUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_service = ConcreteUserService::new();
    let res = user_service.insert(new_user).await;

    if let Err(err) = res {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        ));
    }

    let json_response = serde_json::json!({
        "status": "ok",
        "object": res.unwrap()
    });

    Ok(Json(json_response))
}
