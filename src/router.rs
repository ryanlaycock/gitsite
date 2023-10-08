use crate::{
    models::AppData,
    handlers::{
        get_content_handler,
        get_lib_handler,
        get_tmpl_handler,
        get_index_handler,
        get_index_content_handler,
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
        .route("/content/", get(get_index_content_handler))
        
        .route("/*tmpl_path", get(get_tmpl_handler))
        
        .route("/", get(get_index_handler))
        .with_state(shared_state)
}
