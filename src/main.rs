use std::{path::PathBuf, fmt, env};
use std::error::Error;
use std::collections::BTreeMap;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpRequest, HttpServer, ResponseError};

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

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl ResponseError for CustomError {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_files_dir = env::var("LOCAL_FILES_DIR").expect("LOCAL_FILES_DIR not found");
    let cfg_file = env::var("CONFIG_FILE").expect("CONFIG_FILE not found");

    env_logger::init();

    let site_config = match get_config(&cfg_file) {
        Ok(v) => {
            println!("Parsed config at: {} : {:?}", cfg_file, v);
            Ok(v)
        }
        Err(err) => {
            // Handle the error
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
async fn fetch_file(data: web::Data<AppData>, req: HttpRequest) -> Result<NamedFile, CustomError> {
    println!("Fetching file {} from config {:?}", req.path(), data.site_config);
    
    if let Some(page) = data.site_config.content.get(req.path()) {
        let mut web_path = data.local_files_dir.clone();
        web_path.push_str(&page.filePath);
        println!("Opening {:?}", web_path);
        
        let path: PathBuf = web_path.into();

        if let Ok(file) = NamedFile::open(&path) {
            return Ok(file);
        } else {
            // Value is not a valid file path
            return Err(CustomError("Invalid file path".to_string()));
        }
    }

    Err(CustomError("Key not found".to_string()))
}

#[derive(Debug)]
enum ConfigError {
    FileReadError(std::io::Error),
    YamlParseError(serde_yaml::Error),
    CustomMessage(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileReadError(err) => write!(f, "File read error: {}", err),
            ConfigError::YamlParseError(err) => write!(f, "YAML parse error: {}", err),
            ConfigError::CustomMessage(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for ConfigError {}

fn get_config(file_location: &String) -> Result<Config, ConfigError> {
    let file_str = match std::fs::read_to_string(file_location) {
        Ok(v) => v,
        Err(err) => return Err(ConfigError::FileReadError(err)),
    };

    match serde_yaml::from_str(&file_str) {
        Ok(v) => Ok(v),
        Err(err) => Err(ConfigError::YamlParseError(err)),
    }
}
