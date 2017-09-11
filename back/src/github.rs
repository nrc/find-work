use config::Config;
use data::FetchFile;

use reqwest::{self, header};
use std::collections::HashMap;
use std::iter::FromIterator;

// Client for GitHub API requests.
pub struct Client<'a> {
    reqwest: reqwest::Client,
    config: &'a Config,
    cached_milestones: HashMap<String, HashMap<String, u32>>,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config) -> ::Result<Client<'a>> {
        Ok(Client {
            reqwest: reqwest::Client::new()?,
            config,
            cached_milestones: HashMap::new(),
        })
    }

    fn ensure_milestones(&mut self, repository: &str) -> ::Result<()> {
        if self.cached_milestones.contains_key(repository) {
            return Ok(())
        }

        let query_string = format!("/repos/{}/milestones", repository);
        let map = self.query(
            &query_string,
            |json: Vec<Milestone>| {
                Ok(HashMap::from_iter(json.into_iter().map(|ms| (ms.title, ms.number))))
            }
        )?;

        self.cached_milestones.insert(repository.to_owned(), map);
        Ok(())
    }

    fn milestone_number(&mut self, repository: &str, milestone: &str) -> ::Result<u32> {
        self.ensure_milestones(repository)?;

        let milestones = &self.cached_milestones[repository];
        match milestones.get(milestone) {
            Some(n) => Ok(*n),
            None => Err(::WorkErr(format!("Bad milestone {} in {}", milestone, repository))),
        }
    }

    pub fn fetch_issues(&mut self, repository: &str, labels: &str, milestone: Option<&str>) -> ::Result<Vec<Issue>> {
        let mut query_string = format!("/repos/{}/issues?labels={}", repository, labels);
        if let Some(milestone) = milestone {
            let milestone = self.milestone_number(repository, milestone)?;
            query_string.push_str(&format!("&milestone={}", milestone));
        }
        self.query(
            &query_string,
            |json: Vec<Issue>| { Ok(json) }
        )
    }

    fn query<T, U, F>(&self, query_str: &str, f: F) -> ::Result<T>
    where
        F: FnOnce(U) -> ::Result<T>,
        U: ::serde::de::DeserializeOwned,
    {
        debug!("query: `{}`", query_str);

        let url = format!("https://api.github.com{}", query_str);
        let req = self
            .reqwest
            .get(&url)?
            .header(header::UserAgent::new("nrc"))
            .header(header::Authorization(format!("token {}", self.config.token)))
            .build();
        debug!("request: `{:?}`", req);

        let mut res = self.reqwest.execute(req)?;
        debug!("response: `{:?}`", res);

        if !res.status().is_success() {
            use std::io::Read;

            debug!("Query failed, repsonse: {:?}", res);

            let mut body = String::new();
            res.read_to_string(&mut body)?;
            debug!("body: {}", body);
            return Err(::WorkErr(format!("Server error? {:?}", res.status())));
        }

        let json = res.json()?;
        f(json)
    }
}

impl<'a> FetchFile for Client<'a> {
    fn fetch_file(&self, path: &str) -> ::Result<String> {
        self.query(&format!("/repos/{}/contents/{}", self.config.repository, path), |json: File| {
            if json.type_ != "file" {
                return Err(::WorkErr(format!("Expected file, found {}", json.type_)));
            }
            if json.encoding != "base64" {
                return Err(::WorkErr(format!("Expected base64, found {}", json.encoding)));
            }
            Ok(String::from_utf8(::base64::decode_config(&json.content, ::base64::MIME)?)?)
        })
    }
}

/// A file, returned by the GitHub API.
#[derive(Debug, Deserialize)]
struct File {
    #[serde(rename="type")]
    type_: String,
    name: String,
    content: String,
    encoding: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: u32,
    #[serde(rename="html_url")]
    pub url: String,
    pub title: String,
    pub body: String,
    pub labels: Vec<Label>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Milestone {
    number: u32,
    title: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use mock::mock_config;

    fn mock_client<F>(f: F)
    where F: FnOnce(&mut Client)
    {
        let config = mock_config();
        let mut client = Client::new(&config).unwrap_or_else(|s| panic!("{:?}", s));
        f(&mut client);
    }

    #[test]
    fn test_client_query() {
        #[derive(Debug, Deserialize)]
        struct DirFile {
            #[serde(rename="type")]
            type_: String,
            name: String,
        }

        mock_client(|client| {
            client.query("/repos/nrc/find-work/contents/data", |json: Vec<DirFile>| {
                let file_names: Vec<String> = json.into_iter().map(|f| {
                    assert!(f.type_ == "file");
                    f.name
                }).collect();
                assert!(file_names.contains(&"tabs.json".to_owned()));
                assert!(file_names.contains(&"categories.json".to_owned()));
                assert!(file_names.contains(&"tab-category.json".to_owned()));
                Ok(())
            }).unwrap_or_else(|s| panic!("{:?}", s));
        });
    }

    #[test]
    fn test_fetch_file() {
        mock_client(|client| {
            let contents = client.fetch_file("back/test-token.txt.example").unwrap_or_else(|s| panic!("{:?}", s));
            assert_eq!(contents, "Put your GitHub token here\n");
        });
    }

    #[test]
    fn test_fetch_issues() {
        mock_client(|client| {
            let issues = client.fetch_issues("nrc/testing", "label-1,label-2", None).unwrap_or_else(|s| panic!("{:?}", s));
            assert_eq!(issues.len(), 1);
            assert_eq!(issues[0].number, 2);
            assert_eq!(issues[0].url, "https://github.com/nrc/testing/issues/2");
            assert_eq!(issues[0].title, "Testing 2");
            assert_eq!(issues[0].body, "Another test issue");
            assert_eq!(issues[0].labels.len(), 2);
        });
    }
}
