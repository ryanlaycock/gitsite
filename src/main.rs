mod router;
mod models;
mod handlers;
mod page_domain;

use router::create_router;
use page_domain::get_config_file;
use models::{
    AppData,
    MemoryPage,
};

use std::env;
use std::collections::BTreeMap;

use std::sync::Arc;

use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let local_files_dir = env::var("LOCAL_FILES_DIR");
    let cfg_file = env::var("CONFIG_FILE_PATH").expect("CONFIG_FILE_PATH not found");
    let cfg_file_project = env::var("CONFIG_FILE_GITHUB_PROJECT").expect("CONFIG_FILE_GITHUB_PROJECT not found");
    let github_access_token = env::var("GITHUB_ACCESS_TOKEN").expect("GITHUB_ACCESS_TOKEN not found");
    let github_auth = "Bearer ".to_string() + &github_access_token;

    env_logger::init();

    let site_config = match get_config_file(&cfg_file, cfg_file_project, &github_auth).await {
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
        local_files_dir: local_files_dir.unwrap_or_default(),
        memory_pages: memory_pages,
        github_access_token: github_auth,
    });

    let app = create_router(shared_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
