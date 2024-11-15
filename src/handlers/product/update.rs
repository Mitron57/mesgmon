use crate::application::AppState;
use crate::domain::dto::Description;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_postgres::error::{DbError, SqlState};
use uuid::Uuid;

pub async fn update(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(data): Json<Description>,
) -> (StatusCode, Json<Value>) {
    let result = state
        .product_service
        .update(id, data, state.product_repo.clone(), state.broker.clone())
        .await;
    match result {
        Ok(_) => (StatusCode::OK, Json(Value::default())),
        Err(error) => {
            if let Some(dberr) = error.downcast_ref::<DbError>() {
                if dberr.code() == &SqlState::UNIQUE_VIOLATION {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "user with this data already exists"})),
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
