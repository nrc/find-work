use config::Config;
use data::StructuralData;
use github:: {self, Issue};

use std::collections::HashMap;

pub fn fetch_issues(config: &Config, struct_data: &StructuralData) -> ::Result<IssueData> {
    let mut result = IssueData { issues: HashMap::new() };
    let client = github::Client::new(config)?;
    for tcs in struct_data.tab_category.values() {
        for tc in tcs {
            let category = &struct_data.categories[&tc.category];
            let labels = [&*category.labels, &*tc.labels].concat().join(",");
            let issues = client.fetch_issues(&category.repository, &labels)?;

            if !issues.is_empty() {
                result.issues.insert((tc.tab.clone(), tc.category.clone()), issues);
            }
        }
    }
    Ok(result)
}

#[derive(Debug)]
pub struct IssueData {
    pub issues: HashMap<(String, String), Vec<Issue>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use data::{Category, TabCategory};

    fn mock_config() -> Config {
        Config {
            repository: String::new(),
            username: ::TEST_USERNAME.to_owned(),
            token: ::TEST_TOKEN.to_owned(),
            base_path: String::new(),
            port: 0,
        }
    }

    fn mock_struct_data() -> StructuralData {
        let mut result = StructuralData {
            tabs: HashMap::new(),
            categories: HashMap::new(),
            tab_category: HashMap::new(),
        };

        result.categories.insert("rustfmt".to_owned(), Category {
            id: "rustfmt".to_owned(),
            title: String::new(),
            description: String::new(),
            repository: "rust-lang-nursery/rustfmt".to_owned(),
            labels: vec!["p-high".to_owned()],
            links: vec![],
            tags: vec![],
        });
        result.tab_category.insert("foo".to_owned(), vec![TabCategory {
            tab: "foo".to_owned(),
            category: "rustfmt".to_owned(),
            labels: vec!["bug".to_owned()],
            link: None,
        }]);

        result
    }

    #[test]
    fn test_fetch_isuses() {
        let _data = fetch_issues(&mock_config(), &mock_struct_data()).unwrap();
    }
}
