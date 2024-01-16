use axum::{
    extract::{MatchedPath, Request, State},
    http::StatusCode,
    response::Json,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use axum_macros::debug_handler;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;
mod blueprints;
mod libs;
mod user;
use blueprints::users_controller::{create_user, get_user, list_users};
use diesel::prelude::*;
use libs::pool_creation;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user::schema::users;

use crate::blueprints::users_controller::delete_user;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust-rest-api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = pool_creation();

    let app = Router::new()
        .route("/api", get(version))
        .route("/api/users/all", get(list_users))
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
        .route("/api/users/:id", delete(delete_user))
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        );

    let app = app.fallback(handler_404);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let server = host + ":" + &port;

    // run it
    let listener = tokio::net::TcpListener::bind(server).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn version() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[debug_handler]
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
