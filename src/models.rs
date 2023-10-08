use serde::Deserialize;
use tokio::sync::RwLock;
use std::time::SystemTime;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct Page {
    pub title: String,
    pub file_path: String,
    // child: HashMap<String, Page>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct LibFile {
    pub file_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub lib: BTreeMap<String, LibFile>,
    pub content: BTreeMap<String, Page>,
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