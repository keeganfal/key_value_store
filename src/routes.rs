use crate::handlers;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/store/{key}", get(handlers::get_value))
        .route("/store", post(handlers::set_value))
        .route("/store/{key}", delete(handlers::delete_value))
        .with_state(state)
}
