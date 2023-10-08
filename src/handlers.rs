use crate::models::AppData;
use crate::page_domain::{
    update_and_get_content_page,
    update_and_get_lib_page,
    update_and_get_tmpl_page,
    get_headers,
};

use axum::{
    response::IntoResponse,
    Json,
    extract::{Path, State},
    http::StatusCode
};
use std::sync::Arc;

pub async fn get_lib_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, [(&'static str, &'static str); 1], &'static str)> {
    println!("get_content_handler {:?}", path);
    let app_data = shared_state;
       
    match update_and_get_lib_page(app_data.to_owned(), &path).await {
        Ok(memory_page) => {
            
            return Ok((StatusCode::OK, [("Content-Type", "text")], memory_page.content));
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, [("", "")], ""));
        }
    }    
}

pub async fn get_content_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_data = shared_state;
       
    match update_and_get_content_page(app_data.to_owned(), &path).await {
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

pub async fn get_index_content_handler(State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let app_data = shared_state;
       
    match update_and_get_content_page(app_data.to_owned(), &"index".to_string()).await {
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


pub async fn get_tmpl_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, [(&'static str, &'static str); 1], &'static str)> {
    println!("get_tmpl_handler {:?}", path);
    let mut non_empty_path = path;
    if non_empty_path == "content/" {
        non_empty_path = "index".to_string();
    }
    println!("Request for get_tmpl_handler file at {:?}", non_empty_path);
    let app_data = shared_state;
       
    match update_and_get_tmpl_page(app_data.to_owned(), &non_empty_path).await {
        Ok(memory_page) => {         
            return Ok((StatusCode::OK, [("Content-Type", "text")], memory_page.content));
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, [("", "")], ""));
        }
    }    
}

pub async fn get_index_handler(State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, [(&'static str, &'static str); 1], &'static str)> {
    println!("get_index_handler");
    let app_data = shared_state;
       
    match update_and_get_tmpl_page(app_data.to_owned(), &"index".to_string()).await {
        Ok(memory_page) => {       
            return Ok((StatusCode::OK, [("Content-Type", "text")], memory_page.content));
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, [("", "")], ""));
        }
    }    
}

pub async fn get_header_handler(State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("get_header_handler");
    let app_data = shared_state;

    match get_headers(&app_data) {
        Ok(headers) => {
            let json_response = serde_json::json!({
                "data": headers,
            });
            return Ok((StatusCode::OK, Json(json_response)));
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "error": err.to_string(),
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)))
        } 
    }
}