use crate::models::AppData;
use crate::page_domain::{
    update_and_get_lib_page,
    update_and_get_page,
};

use axum::{
    response::IntoResponse,
    extract::{Path, State},
    http::StatusCode
};
use std::sync::Arc;

pub async fn get_lib_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, [(&'static str, &'static str); 1], &'static str)> {
    println!("get_lib_handler {:?}", path);
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

pub async fn get_page_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, [(&'static str, &'static str); 1], &'static str)> {
    println!("Request for get_page_handler file at {:?}", path);
    let app_data = shared_state;
       
    match update_and_get_page(app_data.to_owned(), &path).await {
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
       
    match update_and_get_page(app_data.to_owned(), &"".to_string()).await {
        Ok(memory_page) => {       
            return Ok((StatusCode::OK, [("Content-Type", "text")], memory_page.content));
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, [("", "")], ""));
        }
    }    
}
