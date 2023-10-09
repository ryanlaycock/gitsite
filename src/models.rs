use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::time::SystemTime;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct Page {
    pub title: String,
    pub file_path: String,
    pub tmpl_html: String,
    pub github_project: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct LibFile {
    pub file_path: String,
    pub github_project: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct HeaderLink {
    pub path: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct Header {
    pub links: Vec<HeaderLink>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub lib: BTreeMap<String, LibFile>,
    pub content: BTreeMap<String, Page>,
    pub header: Header,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemoryPage {
    pub content: String,
    pub last_updated_at: SystemTime,
}

#[derive(Debug)]
pub struct AppData {
    pub site_config: Config,
    pub local_files_dir: String,
    pub memory_pages: RwLock<BTreeMap<String, MemoryPage>>,
}