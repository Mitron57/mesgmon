use crate::application::AppState;
use crate::handlers::Credentials;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_postgres::error::{DbError, SqlState};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(data): Json<Credentials>,
) -> (StatusCode, Json<Value>) {
    let result = state
        .user_service
        .create(data, state.user_repo.clone(), state.broker.clone())
        .await;
    match result {
        Ok(user) => (StatusCode::CREATED, Json(json!(user))),
        Err(error) => {
            if let Some(dberr) = error.downcast_ref::<DbError>() {
                if dberr.code() == &SqlState::UNIQUE_VIOLATION {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "user already exists"})),
                    );
                }
            }
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error.to_string()})),
            )
        }
    }
}
