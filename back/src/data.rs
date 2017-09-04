use std::collections::HashMap;
use std::fs::File;

use serde_json;

const CONFIG_PATH: &'static str = "../data/config.json";

pub const DATA_ROOT: &'static str = "data";
pub const TABS: &'static str = "tabs.json";
pub const CATEGORIES: &'static str = "categories.json";
pub const TAB_CATEGORY: &'static str = "tab-category.json";

/// Configuration for the server.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub repository: String,
    pub username: String,
    pub token: String,
    pub base_path: String,
    pub port: u32,
}

/// Reads a config from CONFIG_PATH.
pub fn read_config() -> ::Result<Config> {
    let file = File::open(CONFIG_PATH)?;
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

// Data for structuring output

#[derive(Clone, Debug)]
pub struct StructuralData {
    pub tabs: HashMap<String, Tab>,
    pub categories: HashMap<String, Category>,
    pub tab_category: HashMap<(String, String), TabCategory>,
}

impl StructuralData {
    pub fn from_raw_data(tabs: &str, categories: &str, tab_category: &str) -> ::Result<StructuralData> {
        Err(::WorkErr("TODO".to_owned()))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub description: String,
    pub repository: String,
    pub labels: Vec<String>,
    pub links: Vec<Link>,
    pub tags: Vec<String>,    
}

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabCategory {
    pub tab: String,
    pub category: String,
    pub labels: Vec<String>,
    pub link: Option<String>,
}
