use config::Config;
use data::{Category, StructuralData, Tab, TabCategory};
use github::Issue;
use issues::IssueData;

use std::collections::{HashMap, HashSet};

pub fn mock_config() -> Config {
    Config {
        repository: "nrc/find-work".to_owned(),
        username: ::TEST_USERNAME.to_owned(),
        token: ::TEST_TOKEN.to_owned(),
        addr: "127.0.0.1:80".to_owned(),
        static_path: String::new(),
        index_path: String::new(),
        dev_mode: false,
    }
}

pub fn mock_struct_data_custom(title: &str, key: &str, repo: &str, labels: Vec<String>, tab_labels: Vec<String>, negative_labels: Option<HashSet<String>>) -> StructuralData {
    let mut result = StructuralData {
        tabs: vec![Tab {
            id: "foo".to_owned(),
            title: "Foo".to_owned(),
            description: "A Foo for foos".to_owned(),
        }],
        categories: HashMap::new(),
        tab_category: HashMap::new(),
    };

    result.categories.insert(key.to_owned(), Category {
        id: key.to_owned(),
        title: title.to_owned(),
        description: String::new(),
        repository: repo.to_owned(),
        labels: labels,
        links: vec![],
        tags: vec!["a".to_owned(), "b".to_owned()],
    });
    result.tab_category.insert("foo".to_owned(), vec![TabCategory {
        tab: "foo".to_owned(),
        category: key.to_owned(),
        labels: tab_labels,
        negative_labels: negative_labels,
        milestone: None,
        link: None,
    }]);

    result
}

pub fn mock_struct_data() -> StructuralData {
    mock_struct_data_custom(
        "Rustfmt",
        "rustfmt",
        "rust-lang-nursery/rustfmt",
        vec!["p-high".to_owned()],
        vec!["bug".to_owned()],
        None,
    )
}

pub fn mock_issue_data() -> IssueData {
    let mut issues = HashMap::new();
    issues.insert(("foo".to_owned(), "rustfmt".to_owned()), vec![Issue {
        number: 42,
        url: String::new(),
        title: "Title".to_owned(),
        body: "body/description".to_owned(),
        labels: vec![],
    }]);
    IssueData {
        issues,
    }
}
