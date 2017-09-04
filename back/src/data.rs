use config::Config;
use github::Client;

use std::collections::HashMap;

use serde_json;

const DATA_ROOT: &'static str = "data";
const TABS: &'static str = "tabs.json";
const CATEGORIES: &'static str = "categories.json";
const TAB_CATEGORY: &'static str = "tab-category.json";


// Data for structuring output
pub fn fetch_structural_data(config: &Config) -> ::Result<StructuralData> {
    let client = Client::new(config)?;

    let tabs = client.fetch_file(&format!("{}/{}", DATA_ROOT, TABS))?;
    let categories = client.fetch_file(&format!("{}/{}", DATA_ROOT, CATEGORIES))?;
    let tab_category = client.fetch_file(&format!("{}/{}", DATA_ROOT, TAB_CATEGORY))?;
    
    let data = StructuralData::from_raw_data(&tabs, &categories, &tab_category)?;
    Ok(data)
}

#[derive(Clone, Debug, Default)]
pub struct StructuralData {
    pub tabs: HashMap<String, Tab>,
    pub categories: HashMap<String, Category>,
    pub tab_category: HashMap<(String, String), TabCategory>,
}

impl StructuralData {
    pub fn from_raw_data(tabs: &str, categories: &str, tab_category: &str) -> ::Result<StructuralData> {
        let tabs: Vec<Tab> = serde_json::from_str(tabs)?;
        let categories: Vec<Category> = serde_json::from_str(categories)?;
        let tab_category: Vec<TabCategory> = serde_json::from_str(tab_category)?;

        let mut result = StructuralData::default();
        for t in tabs {
            result.tabs.insert(t.id.clone(), t);
        }
        for c in categories {
            result.categories.insert(c.id.clone(), c);
        }
        for tc in tab_category {
            result.tab_category.insert((tc.tab.clone(), tc.category.clone()), tc);
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

#[cfg(test)]
mod test {
    use super::*;

    fn mock_config() -> Config {
        Config {
            repository: "nrc/find-work".to_owned(),
            username: ::TEST_USERNAME.to_owned(),
            token: ::TEST_TOKEN.to_owned(),
            base_path: String::new(),
            port: 0,
        }
    }

    #[test]
    fn test_fetch_structural_data() {
        let data = fetch_structural_data(&mock_config()).unwrap();
        assert!(data.tabs.contains_key("starters"));
        assert!(data.categories.contains_key("rustfmt"));
        assert!(data.tab_category.contains_key(&("starters".to_owned(), "rustfmt".to_owned())));
    }
}