use data::Config;

use reqwest::{self, header};

// Client for GitHub API requests.
pub struct Client<'a> {
    reqwest: reqwest::Client,
    config: &'a Config,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config) -> ::Result<Client<'a>> {
        Ok(Client {
            reqwest: reqwest::Client::new()?,
            config
        })
    }

    pub fn fetch_issues(&self, repository: &'a str, labels: &str) -> ::Result<Vec<Issue>> {
        self.query(
            &format!("/repos/{}/issues?labels={}", repository, labels),
            |json: Vec<Issue>| { Ok(json) }
        )
    }

    pub fn fetch_file(&self, path: &str) -> ::Result<String> {
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

        // For debugging, print the body of the response.
        // let mut s = String::new();
        // use std::io::Read;
        // res.read_to_string(&mut s);
        // println!("body: {}", s);

        if !res.status().is_success() {
            debug!("Query failed, repsonse: {:?}", res);
            return Err(::WorkErr(format!("Server error? {:?}", res.status())));
        }

        let json = res.json()?;
        f(json)
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

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub number: u32,
    #[serde(rename="html_url")]
    pub url: String,
    pub title: String,
    pub body: String,
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub url: String,
    pub name: String,
    pub color: String,
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

    fn mock_client<F>(f: F)
    where F: FnOnce(&Client)
    {
        let config = mock_config();
        let client = Client::new(&config).unwrap_or_else(|s| panic!("{:?}", s));
        f(&client);
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
            let issues = client.fetch_issues("nrc/testing", "label-1,label-2").unwrap_or_else(|s| panic!("{:?}", s));
            assert_eq!(issues.len(), 1);
            assert_eq!(issues[0].number, 2);
            assert_eq!(issues[0].url, "https://github.com/nrc/testing/issues/2");
            assert_eq!(issues[0].title, "Testing 2");
            assert_eq!(issues[0].body, "Another test issue");
            assert_eq!(issues[0].labels.len(), 2);
        });
    }
}
