use data::{StructuralData, Link};
use github::Issue;
use issues::IssueData;

use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct Blob {
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Serialize)]
pub struct Tab {
    pub title: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Category {
    pub title: String,
    pub description: String,
    pub links: Vec<Link>,
    pub tags: Vec<String>,
    pub issues: Vec<Issue>,
}

impl Blob {
    /// Make a blob from the strucrtural data and issues we've pulled from GitHub.
    pub fn make(struct_data: &StructuralData, issues: &IssueData) -> ::Result<Blob> {
        let mut result = Blob { tabs: vec![] };
        // TODO can struct_data.tabs be a Vec?

        // Iterate over tabs, a raw tab becomes a blob tab.
        for t in struct_data.tabs.values() {
            let mut tab = Tab {
                title: t.title.clone(),
                description: t.description.clone(),
                categories: vec![],
                tags: vec![],
            };

            // We'll collect a list of tags for each tab.
            let mut tags = HashSet::new();

            // Iterate over the categories in each tab.
            let tcs = &struct_data.tab_category[&t.id];
            for tc in tcs {
                assert!(tc.tab == t.id);
                let cat = &struct_data.categories[&tc.category];
                assert!(cat.id == tc.category);

                // If there are no issues, don't list the category.
                if let Some(issues) = issues.issues.get(&(tc.tab.clone(), tc.category.clone())) {
                    assert!(!issues.is_empty());
                    // Merge the various links into a single list.
                    let links = tc
                        .link
                        .iter()
                        .cloned()
                        .chain(Some(Link {
                            text: "repository".to_owned(),
                            url: format!("https://github.com/{}", cat.repository),
                        }).into_iter())
                        .chain(cat.links.iter().cloned())
                        .collect();
                    let category = Category {
                        title: cat.title.clone(),
                        description: cat.description.clone(),
                        links,
                        tags: cat.tags.clone(),
                        issues: issues.clone(),
                    };
                    tab.categories.push(category);
                    tags.extend(&cat.tags);
                }
            }


            tab.tags = tags.into_iter().cloned().collect();
            tab.tags.sort();

            result.tabs.push(tab);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use data::{Category, Tab, TabCategory};
    use github::Issue;
    use std::collections::HashMap;

    // TODO factor out mock_* fns into their own module
    fn mock_struct_data() -> StructuralData {
        let mut result = StructuralData {
            tabs: HashMap::new(),
            categories: HashMap::new(),
            tab_category: HashMap::new(),
        };

        result.tabs.insert("foo".to_owned(), Tab {
            id: "foo".to_owned(),
            title: "Foo".to_owned(),
            description: "A Foo for foos".to_owned(),
        });
        result.categories.insert("rustfmt".to_owned(), Category {
            id: "rustfmt".to_owned(),
            title: "Rustfmt".to_owned(),
            description: String::new(),
            repository: "rust-lang-nursery/rustfmt".to_owned(),
            labels: vec!["p-high".to_owned()],
            links: vec![],
            tags: vec!["a".to_owned(), "b".to_owned()],
        });
        result.tab_category.insert("foo".to_owned(), vec![TabCategory {
            tab: "foo".to_owned(),
            category: "rustfmt".to_owned(),
            labels: vec!["bug".to_owned()],
            link: None,
        }]);

        result
    }

    fn mock_issue_data() -> IssueData {
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

    #[test]
    fn test_make() {
        let blob = Blob::make(&mock_struct_data(), &mock_issue_data()).unwrap_or_else(|s| panic!("{:?}", s));
        assert_eq!(blob.tabs.len(), 1);
        let tab = &blob.tabs[0];
        assert_eq!(tab.title, "Foo");
        assert_eq!(tab.tags, &["a".to_owned(), "b".to_owned()]);
        assert_eq!(tab.categories.len(), 1);
        let cat = &tab.categories[0];
        assert_eq!(cat.title, "Rustfmt");
    }
}
