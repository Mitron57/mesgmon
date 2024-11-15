use crate::application::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

pub async fn delete(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Value>) {
    let result = state
        .user_service
        .delete(id, state.user_repo.clone(), state.broker.clone())
        .await;
    match result {
        Ok(_) => (StatusCode::NO_CONTENT, Json(Value::default())),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}
