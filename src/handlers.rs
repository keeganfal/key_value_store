use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct GetResponse {
    pub key: String,
    pub value: Option<String>,
}

pub async fn get_value(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Json<GetResponse> {
    let store = state.store.read().unwrap();
    let value = store.get(&key);
    Json(GetResponse { key, value })
}

pub async fn set_value(
    State(state): State<AppState>,
    Json(payload): Json<SetRequest>,
) -> StatusCode {
    let mut store = state.store.write().unwrap();
    store.set(payload.key, payload.value);
    StatusCode::CREATED
}

pub async fn delete_value(State(state): State<AppState>, Path(key): Path<String>) -> StatusCode {
    let mut store = state.store.write().unwrap();
    store.delete(&key);
    StatusCode::OK
}
