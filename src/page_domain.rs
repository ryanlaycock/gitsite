use std::collections::BTreeMap;
use std::fs;
use std::error::Error;
use crate::models::{
    AppData,
    MemoryPage,
    Config,
    Page,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum FileError {
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

pub async fn update_and_get_lib_page(app_data: Arc<AppData>, path: &String) -> Result<MemoryPage, FileError>{
    if let Some(page) = app_data.site_config.lib.get(path) {
        return update_and_get_in_memory(&app_data, path, &page.file_path).await;
    } else {
        // File not defined in config
        return Err(FileError::FileNotFound())
    }
}

pub async fn update_and_get_content_page(app_data: Arc<AppData>, path: &String) -> Result<MemoryPage, FileError>{
    if let Some(page) = app_data.site_config.content.get(path) {
        return update_and_get_in_memory(&app_data, path, &page.file_path).await;
    } else {
        // File not defined in config
        return Err(FileError::FileNotFound())
    }
}

pub async fn update_and_get_tmpl_page(app_data: Arc<AppData>, path: &String) -> Result<MemoryPage, FileError>{
    if let Some(page) = app_data.site_config.content.get(path) {
        if let Some(lib_page) = app_data.site_config.lib.get(&page.tmpl_html) {
            // If requested path is specified, and tmplHtml is specified, load it
            return update_and_get_in_memory(&app_data, &page.tmpl_html, &lib_page.file_path).await;
        } else {
            // File not defined in config
            return Err(FileError::FileNotFound())
        }
    } else {
        // File not defined in config
        return Err(FileError::FileNotFound())
    }
}

async fn update_and_get_in_memory(app_data: &Arc<AppData>, path: &String, content_file_path: &String) -> Result<MemoryPage, FileError> {
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
    local_file_str.push_str(content_file_path.as_str());
    println!("Fetching locally {:?} at location {:?}", content_file_path, local_file_str);
    match get_local_file_string(local_file_str) {
        Ok(file_string) => {
            let new_memory_page: MemoryPage = MemoryPage { content: file_string, last_updated_at: SystemTime::now() };
            app_data.memory_pages.write().await.insert(path.to_string(), new_memory_page.clone());
            println!("Memory page after cache {:?}", app_data);
            return Ok(new_memory_page);
        },
        Err(_) => return Err(FileError::FileNotFound())
    };

}

pub fn get_local_config(file_location: &String) -> Result<Config, FileError> {
    let file_str = match std::fs::read_to_string(file_location) {
        Ok(v) => v,
        Err(err) => return Err(FileError::FileReadError(err)),
    };

    match serde_yaml::from_str(&file_str) {
        Ok(v) => Ok(v),
        Err(err) => Err(FileError::YamlParseError(err)),
    }
}

fn get_local_file_string(path: String) -> Result<String, FileError> {
    match fs::read_to_string(path) {
        Ok(v) => return Ok(v),
        Err(_) => return Err(FileError::LocalFileNotFound())
    };
}
