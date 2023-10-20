use crate::{
    models::AppData,
    handlers,
};

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn create_router(shared_state: Arc<AppData>) -> Router {
    Router::new()
        // Page and content loading handlers
        .route("/lib/*lib_path", get(handlers::get_lib_handler))
        
        .route("/*tmpl_path", get(handlers::get_page_handler))
        
        .route("/", get(handlers::get_index_handler))

        // API handlers
        .route("/api/header/links", get(handlers::get_header_handler))
        .with_state(shared_state)
}
