use config::Config;
use data::StructuralData;
use github:: {self, Issue};

use std::collections::HashMap;

pub fn fetch_issues(config: &Config, struct_data: &StructuralData) -> ::Result<IssueData> {
    let mut result = IssueData { issues: HashMap::new() };
    let mut client = github::Client::new(config)?;
    for tcs in struct_data.tab_category.values() {
        for tc in tcs {
            let category = &struct_data.categories[&tc.category];
            let labels = [&*category.labels, &*tc.labels].concat().join(",");
            let negative_labels = tc.negative_labels.as_ref();
            let issues = client.fetch_issues(&category.repository, &labels, tc.milestone.as_ref().map(|s| &**s))?;

            let issues = if let Some(negative_labels) = negative_labels {
                issues.into_iter()
                .filter(|issue| issue.labels.iter().all(|l| !negative_labels.contains(&l.name)))
                .collect()
            } else {
                issues
            };

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
    use mock::{mock_config, mock_struct_data};

    #[test]
    fn test_fetch_isuses() {
        let _data = fetch_issues(&mock_config(), &mock_struct_data()).unwrap();
    }
}
