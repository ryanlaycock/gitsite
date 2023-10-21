use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::time::SystemTime;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Page {
    pub title: String,
    pub file_path: String,
    pub tmpl_html: String,
    pub github_project: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub pinned_posts: Option<Vec<String>>,
    pub recache_seconds: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct LibFile {
    pub file_path: String,
    pub github_project: Option<String>,
    pub recache_seconds: Option<u64>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct SiteConfig {
    pub default_recache_seconds: u64,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct Config {
    pub lib: BTreeMap<String, LibFile>,
    pub content: BTreeMap<String, Page>,
    pub header: Header,
    pub site_config: SiteConfig,
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