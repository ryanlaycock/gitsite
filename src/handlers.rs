use crate::models::AppData;
use crate::page_domain::update_and_get_page;

use axum::{
    response::IntoResponse,
    Json,
    extract::{Path, State},
    http::StatusCode,
};


use std::sync::Arc;

pub async fn get_content_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Request for content at {:?}", path);
    let app_data = shared_state;
       
    match update_and_get_page(app_data, &path).await {
        Ok(memory_page) => {
            let json_response = serde_json::json!({
                "data": memory_page.content,
                "lastUpdatedAt": memory_page.last_updated_at,
            });
            return Ok((StatusCode::OK, Json(json_response)));
        },
        Err(err) => {
            let error_response = serde_json::json!({
                "message": err.to_string(),
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}