use crate::{
    models::AppData,
    handlers::{
        get_content_handler,
        get_lib_handler,
    }
};

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn create_router(shared_state: Arc<AppData>) -> Router {
    Router::new()
        .route("/lib/*lib_path", get(get_lib_handler))
        .route("/content/*content_path", get(get_content_handler))
        .with_state(shared_state)
}
