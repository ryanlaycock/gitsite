use std::{path::PathBuf, fmt, env, io};
use std::error::Error;
use std::collections::BTreeMap;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpServer, ResponseError, HttpResponse, middleware};

use serde::Deserialize;
use serde_yaml::{self};

use std::sync::Arc;

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

#[derive(Debug, Clone)]
struct AppData {
    site_config: Config,
    local_files_dir: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let app_data = AppData { 
        site_config: site_config.unwrap(),
        local_files_dir: local_files_dir,
    };

    println!("{:?}", app_data);
    println!("{:?}", web::Data::new(app_data.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(fetch_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/{path:.*}")]
async fn fetch_file(data: web::Data<AppData>, req: HttpRequest) -> Result<NamedFile, CustomActixError> {
    println!("GET request on: {}", req.path());

    match get_local_file_path(data.into_inner(), req.path().to_string()) {
        Ok(v) => {
            println!("found local file path: {} for path: {}", v.display(), req.path());
            match NamedFile::open(v) {
                Ok(v) => return Ok(v),
                Err(err) => return Err(CustomActixError(err.to_string())),
            }
        }
        Err(_) => {
            println!("Could not find file: {} Redirect to 404 NOT_FOUND", req.path());
            return Err(CustomActixError("Invalid file path".to_string()))
        }
    }
}

fn get_local_file_path(data: Arc<AppData>, path: String) -> Result<PathBuf, FileError> {
    if let Some(page) = data.site_config.content.get(&path) {
        let mut web_path = data.local_files_dir.clone();
        web_path.push_str(&page.filePath);
        
        return Ok(web_path.into())
    }
    Err(FileError::LocalFileNotFound())
}

#[derive(Debug)]
enum FileError {
    FileReadError(std::io::Error),
    YamlParseError(serde_yaml::Error),
    LocalFileNotFound(),
    CustomMessage(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::FileReadError(err) => write!(f, "File read error: {}", err),
            FileError::YamlParseError(err) => write!(f, "YAML parse error: {}", err),
            FileError::LocalFileNotFound() => write!(f, "{}", "local file not found"),
            FileError::CustomMessage(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for FileError {}

#[derive(Debug)]
struct CustomActixError(String);

impl fmt::Display for CustomActixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl ResponseError for CustomActixError {}

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

fn cache_local_file(req: HttpRequest) -> io::Result<()> {
    if !std::path::Path::new(&local_file_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Local file not found",
        ));
    }

    std::fs::create_dir_all(&destination_dir)?;

    let local_file_name = std::path::Path::new(&local_file_path)
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;
    let destination_file_path = std::path::Path::new(&destination_dir).join(&local_file_name);

    // Copy the file from the source to the destination
    std::fs::copy(local_file_path, &destination_file_path)?;

    Ok(())
}