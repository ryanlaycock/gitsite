use std::fs::File;
use std::{path::PathBuf};
use std::fmt;
use std::collections::BTreeMap;

use actix_files::NamedFile;
use actix_web::{get, web, App, Error, HttpRequest, HttpServer, ResponseError};

use serde::Deserialize;
use serde_yaml::{self};

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
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl ResponseError for CustomError {}


#[get("/{path:.*}")]
async fn fetch_file(data: web::Data<AppData>, req: HttpRequest) -> Result<NamedFile, CustomError> {
    println!("Fecthing file {} from config {:?}", req.path(), data.site_config);
    
    if let Some(page) = data.site_config.content.get(req.path()) {
        let web_path = "/home/ryan/gh-rl/github-website/".to_string() + &page.filePath;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config_str: String = get_config_str().unwrap();
    let result: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&config_str);
    let site_config = result.unwrap();
    let app_data = AppData { site_config: site_config };

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

fn get_config_str() -> Result<String, Error> {
    let web_path = "/home/ryan/gh-rl/github-website/config.yaml";
    let f = std::fs::read_to_string(web_path);
    let file_str = match f {
        Ok(v) => v,
        Err(e) => {
            println!("Error {}", e);
            return Err(e.into());
        }
    };
    Ok(file_str)
}
