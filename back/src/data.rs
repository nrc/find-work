use std::collections::HashMap;

const CONFIG_PATH: &'static str = "../data/config.json";
const DATA_ROOT: &'static str = "/data";

const TABS: &'static str = "tabs.json";
const CATEGORIES: &'static str = "categories.json";
const TAB_CATEGORY: &'static str = "tab-category.json";

/// Configuration for the server.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub repsitory: String,
    pub base_path: String,
    pub port: u32,
}

// Data for structuring output

#[derive(Clone, Debug)]
pub struct StructuralData {
    pub tabs: HashMap<String, Tab>,
    pub categories: HashMap<String, Category>,
    pub tab_categories: HashMap<(String, String), TabCategory>,
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
