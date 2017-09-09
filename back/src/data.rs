use config::Config;
use github::Client;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_json;

const DATA_ROOT: &'static str = "data";
const TABS: &'static str = "tabs.json";
const CATEGORIES: &'static str = "categories.json";
const TAB_CATEGORY: &'static str = "tab-category.json";


// Data for structuring output
pub fn fetch_structural_data(config: &Config) -> ::Result<StructuralData> {
    if config.dev_mode {
        make_structural_data(LocalFileLoader)
    } else {
        make_structural_data(Client::new(config)?)
    }
}

fn make_structural_data<F: FetchFile>(loader: F) -> ::Result<StructuralData> {
    let tabs = loader.fetch_file(&format!("{}/{}", DATA_ROOT, TABS))?;
    let categories = loader.fetch_file(&format!("{}/{}", DATA_ROOT, CATEGORIES))?;
    let tab_category = loader.fetch_file(&format!("{}/{}", DATA_ROOT, TAB_CATEGORY))?;
    
    let data = StructuralData::from_raw_data(&tabs, &categories, &tab_category)?;
    Ok(data)
}

// Load the contents of a file from somewhere.
pub trait FetchFile {
    fn fetch_file(&self, filename: &str) -> ::Result<String>;
}

struct LocalFileLoader;

impl FetchFile for LocalFileLoader {
    fn fetch_file(&self, filename: &str) -> ::Result<String> {
        let mut file = File::open(filename)?;
        let mut result = String::new();
        file.read_to_string(&mut result)?;
        Ok(result)
    }
}

#[derive(Clone, Debug, Default)]
pub struct StructuralData {
    pub tabs: Vec<Tab>,
    pub categories: HashMap<String, Category>,
    pub tab_category: HashMap<String, Vec<TabCategory>>,
}

impl StructuralData {
    fn from_raw_data(tabs: &str, categories: &str, tab_category: &str) -> ::Result<StructuralData> {
        let tabs: Vec<Tab> = serde_json::from_str(tabs)?;
        let categories: Vec<Category> = serde_json::from_str(categories)?;
        let tab_category: Vec<TabCategory> = serde_json::from_str(tab_category)?;

        let mut result = StructuralData::default();
        result.tabs = tabs;
        for c in categories {
            result.categories.insert(c.id.clone(), c);
        }
        for tc in tab_category {
            result.tab_category.entry(tc.tab.clone()).or_insert(vec![]).push(tc.clone());
        }

        Ok(result)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TabCategory {
    pub tab: String,
    pub category: String,
    pub labels: Vec<String>,
    pub link: Option<Link>,
}

#[cfg(test)]
mod test {
    use super::*;
    use mock::mock_config;

    #[test]
    fn test_fetch_structural_data() {
        let data = fetch_structural_data(&mock_config()).unwrap();
        assert!(data.tabs.iter().any(|t| t.id == "starters"));
        assert!(data.categories.contains_key("rustfmt"));
        assert!(data.tab_category.contains_key("starters"));
    }

    #[test]
    fn test_local_file_loader() {
        let loader = LocalFileLoader;
        let s = loader.fetch_file("test-token.txt.example").unwrap();
        assert_eq!(s, "Put your GitHub token here\n");
    }
}
