use std::f32::consts::E;
use std::{path::PathBuf, fmt, env, io};
use std::error::Error;
use std::collections::BTreeMap;
use std::fs;
use std::time::{Duration, SystemTime};

use serde::Deserialize;
use serde_yaml::{self};
use serde_json::{self};

use std::sync::Arc;

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use tokio::sync::RwLock;

#[derive(Deserialize, Debug, Clone)]
struct Page {
    title: String,
    filePath: String,
    // child: HashMap<String, Page>,
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    content: BTreeMap<String, Page>,
}

#[derive(Deserialize, Debug, Clone)]
struct MemoryPage {
    content: String,
    last_updated_at: SystemTime,
}

#[derive(Debug)]
struct AppData {
    site_config: Config,
    local_files_dir: String,
    memory_pages: RwLock<BTreeMap<String, MemoryPage>>,
}

#[tokio::main]
async fn main() {
    let now = SystemTime::now();
    let local_files_dir = env::var("LOCAL_FILES_DIR").expect("LOCAL_FILES_DIR not found");
    let cfg_file = env::var("CONFIG_FILE").expect("CONFIG_FILE not found");

    env_logger::init();

    let site_config = match get_local_config(&cfg_file) {
        Ok(v) => {
            println!("Parsed config at: {} : {:?}", cfg_file, v);
            Ok(v)
        }
        Err(err) => {
            eprintln!("Could not parse config at: {}: {}", cfg_file, err);
            Err(err)
        }
    };

    
    let memory_pages: RwLock<BTreeMap<String, MemoryPage>> = RwLock::new(BTreeMap::new());

    let shared_state = Arc::new(AppData { 
        site_config: site_config.unwrap(),
        local_files_dir: local_files_dir,
        memory_pages: memory_pages,
    });

    println!("{:?}", shared_state);

    let app = create_router(shared_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

fn create_router(shared_state: Arc<AppData>) -> Router {

    Router::new()
        .route("/content/*content_path", get(get_content_handler))
        .with_state(shared_state)
}

async fn get_content_handler(Path(path): Path<String>, State(shared_state): State<Arc<AppData>>) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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

async fn update_and_get_page(app_data: Arc<AppData>, path: &String) -> Result<MemoryPage, FileError> {
    println!("Looking in memory {:?} for {:?} in {:?}", app_data.memory_pages, path, app_data.site_config.content);
    if let Some(page) = app_data.site_config.content.get(path) {
        println!("Found page {:?}", page);

        // Check if we already have this cached (within 10 seconds)
        if let Some(memory_page) = app_data.memory_pages.read().await.get(path) {
            println!("Already cached memory_page {:?}", memory_page);
            if SystemTime::now().duration_since(memory_page.last_updated_at).unwrap_or(Duration::from_secs(0)) < Duration::from_secs(10) {
                return Ok(memory_page.to_owned());
            }
        }
        
        // TODO add check on local file OR GitHub file

        // Update and return from a local file
        let mut local_file_str = app_data.local_files_dir.clone();
        local_file_str.push_str(page.filePath.as_str());
        println!("Fetching locally {:?} at location {:?}", page, local_file_str);
        match get_local_file_string(local_file_str) {
            Ok(file_string) => {
                let new_memory_page: MemoryPage = MemoryPage { content: file_string, last_updated_at: SystemTime::now() };
                app_data.memory_pages.write().await.insert(path.to_string(), new_memory_page.clone());
                println!("Memory page after cache {:?}", app_data);
                return Ok(new_memory_page);
            },
            Err(_) => return Err(FileError::FileNotFound())
        };

        // TODO Update and return from GitHub
    } else {
        // File not defined in config
        return Err(FileError::FileNotFound())
    }
}

fn get_local_file_string(path: String) -> Result<String, FileError> {
    match fs::read_to_string(path) {
        Ok(v) => return Ok(v),
        Err(_) => return Err(FileError::LocalFileNotFound())
    };
}

#[derive(Debug)]
enum FileError {
    FileReadError(std::io::Error),
    YamlParseError(serde_yaml::Error),
    LocalFileNotFound(),
    FileNotFound(),
    CustomMessage(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::FileReadError(err) => write!(f, "File read error: {}", err),
            FileError::YamlParseError(err) => write!(f, "YAML parse error: {}", err),
            FileError::LocalFileNotFound() => write!(f, "{}", "local file not found"),
            FileError::FileNotFound() => write!(f, "{}", "file not found on the server"),
            FileError::CustomMessage(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for FileError {}

fn get_local_config(file_location: &String) -> Result<Config, FileError> {
    let file_str = match std::fs::read_to_string(file_location) {
        Ok(v) => v,
        Err(err) => return Err(FileError::FileReadError(err)),
    };

    match serde_yaml::from_str(&file_str) {
        Ok(v) => Ok(v),
        Err(err) => Err(FileError::YamlParseError(err)),
    }
}
