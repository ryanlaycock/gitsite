use std::fs;
use std::error::Error;
use crate::models::{
    AppData,
    MemoryPage,
    Config,
    HeaderLink, HeaderSocial,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use axum::body::StreamBody;
use reqwest::header;
use handlebars::{Handlebars, to_json, RenderError};
use serde::Serialize;

#[derive(Debug)]
pub enum FileError {
    FileReadError(std::io::Error),
    YamlParseError(serde_yaml::Error),
    LocalFileNotFound(),
    FileNotFound(),
    CustomMessage(String),
}

#[derive(Debug, Serialize)]
struct TemplateContent {
    content: String,
    header_links: Vec<HeaderLink>,
    header_socials: Vec<HeaderSocial>,
    site_title: String,
    title: String,
    pinned_posts: Vec<PinnedPost>,
}

#[derive(Debug, Serialize)]
struct PinnedPost {
    title: String,
    description: Option<String>,
    date: Option<String>,
    link: String,
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
        return update_and_get_in_memory(
            &app_data,
            path,
            &page.file_path,
            &page.github_project,
            get_recache_seconds(page.recache_seconds, app_data.site_config.site_config.default_recache_seconds))
            .await;
    } else {
        // File not defined in config
        return Err(FileError::FileNotFound())
    }
}

pub async fn update_and_get_page(app_data: Arc<AppData>, path: &String) -> Result<MemoryPage, FileError>{
    if let Some(page) = app_data.site_config.content.get(path) {
        if let Some(lib_page) = app_data.site_config.lib.get(&page.tmpl_html) {
            let content_string = update_and_get_in_memory(
                &app_data,
                path,
                &page.file_path,
                &page.github_project,
                get_recache_seconds(page.recache_seconds, app_data.site_config.site_config.default_recache_seconds))
                .await;
            if content_string.is_err() {
                return Err(FileError::FileNotFound())
            }

            // If requested path is specified, and tmplHtml is specified, load it
            let lib_file_string = update_and_get_in_memory(
                &app_data,
                &page.tmpl_html,
                &lib_page.file_path,
                &lib_page.github_project,
                get_recache_seconds(lib_page.recache_seconds, app_data.site_config.site_config.default_recache_seconds))
                .await;
            if lib_file_string.is_err() {
                return Err(FileError::FileNotFound())
            }

            match inject_content(
                &lib_file_string.as_ref().unwrap().content,
                &content_string.unwrap().content,
                page.pinned_posts.clone(),
                page.title.clone(),
                &app_data) 
            {
                Ok(injected) => return Ok(MemoryPage{content: injected, last_updated_at: lib_file_string.unwrap().last_updated_at}),
                Err(_) => return Err(FileError::FileNotFound())
            }
        } else {
            // Lib file not defined in config
            return Err(FileError::FileNotFound())
        }
    } else {
        // Config file not defined in config
        return Err(FileError::FileNotFound())
    }
}

fn inject_content(tmpl_file: &String, config_string: &String, pinned_post_links_option: Option<Vec<String>>, title: String, app_data: &Arc<AppData>) -> Result<String, RenderError> {
    let mut reg = Handlebars::new();

    let mut pinned_posts: Vec<PinnedPost> = vec![];
    let pinned_post_links = pinned_post_links_option.unwrap_or_default();
    for post_link in pinned_post_links.iter() {
        if let Some(page) = app_data.site_config.content.get(post_link) {
            pinned_posts.push(PinnedPost{
                title: page.title.clone(),
                description: page.description.clone(),
                date: page.date.clone(),
                link: post_link.to_owned(),
            });
        }
    }

    let template_config = TemplateContent{
        content: config_string.to_string(),
        header_links: app_data.site_config.header.links.clone(),
        header_socials: app_data.site_config.header.socials.clone().unwrap_or_default(),
        title: title,
        site_title: app_data.site_config.site_config.title.clone(),
        pinned_posts: pinned_posts,
    };

    // register template using given name
    reg.register_template_string("tmpl", tmpl_file)?;
    return reg.render("tmpl", &to_json(template_config));
}

fn get_recache_seconds(page_recache_seconds: Option<u64>, default_recache_seconds: u64) -> u64 {
    if let Some(page_recache_seconds) = page_recache_seconds {
        return page_recache_seconds;
    }
    return default_recache_seconds;
}

async fn update_and_get_in_memory(app_data: &Arc<AppData>, path: &String, content_file_path: &String, github_project: &Option<String>, cache_time: u64) -> Result<MemoryPage, FileError> {
    // Check if we already have this cached
    if let Some(memory_page) = app_data.memory_pages.read().await.get(path) {
        println!("Already cached memory_page {:?}", path);
        if SystemTime::now().duration_since(memory_page.last_updated_at).unwrap_or(Duration::from_secs(0)) < Duration::from_secs(cache_time) {
            return Ok(memory_page.to_owned());
        }
        // TODO Else return the old page but run this fetch in the background
        // No need to make the user wait for the reload
    }
    
    // Check if we should get this from GitHub
    if let Some(github_project_string) = github_project {
        match get_github_file_string(github_project_string.to_owned(), content_file_path.to_owned(), app_data.github_access_token.clone()).await {
            Ok(file_string) => {
                return Ok(cache_file_string(app_data, path.to_string(), file_string).await);
            },
            Err(_) => return Err(FileError::FileNotFound())
        };
    }

    // Update and return from a local file
    let mut local_file_str = app_data.local_files_dir.clone();
    local_file_str.push_str(content_file_path.as_str());
    println!("Fetching locally {:?} at location {:?}", content_file_path, local_file_str);
    match get_local_file_string(local_file_str) {
        Ok(file_string) => {
            return Ok(cache_file_string(app_data, path.to_string(), file_string).await);
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

async fn cache_file_string(app_data: &Arc<AppData>, path: String, file_string: String) -> MemoryPage {
    let new_memory_page: MemoryPage = MemoryPage { content: file_string, last_updated_at: SystemTime::now() };
    app_data.memory_pages.write().await.insert(path.to_string(), new_memory_page.clone());
    return new_memory_page;
}

async fn get_github_file_string(project: String, path: String, access_token: String) -> Result<String, reqwest::Error> {
    let source = format!("https://api.github.com/repos/{}/contents/{}", project, path);
    println!("Requesting path {:?} from GitHub with request {:?}", path, source);
    let accept_header: String;
    if path.ends_with("html") {
        accept_header = "application/vnd.github.raw".to_string();
    } else {
        accept_header = "application/vnd.github.html".to_string();
    }
    let client = reqwest::Client::new();
    match client
        .get(source)
        .header(header::USER_AGENT, "gitsite")
        .header(header::ACCEPT, accept_header)
        .header(header::AUTHORIZATION, access_token)
        .send()
        .await?.text().await {
        Ok(resp) => {
            return Ok(resp);
        },
        Err(err) => return Err(err),
    }
}

fn get_local_file_string(path: String) -> Result<String, FileError> {
    match fs::read_to_string(path) {
        Ok(v) => return Ok(v),
        Err(_) => return Err(FileError::LocalFileNotFound())
    };
}
