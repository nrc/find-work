use data::{self, Config, StructuralData};

use reqwest::{self, header};

pub fn fetch_structural_data(config: &Config) -> ::Result<StructuralData> {
    let client = Client::new(config)?;

    let tabs = client.fetch_file(&format!("{}/{}", data::DATA_ROOT, data::TABS))?;
    let categories = client.fetch_file(&format!("{}/{}", data::DATA_ROOT, data::CATEGORIES))?;
    let tab_category = client.fetch_file(&format!("{}/{}", data::DATA_ROOT, data::TAB_CATEGORY))?;
    
    let data = StructuralData::from_raw_data(&tabs, &categories, &tab_category)?;
    Ok(data)
}


// Client for GitHub API requests.
struct Client<'a> {
    reqwest: reqwest::Client,
    config: &'a Config,
}

impl<'a> Client<'a> {
    fn new(config: &'a Config) -> ::Result<Client<'a>> {
        Ok(Client {
            reqwest: reqwest::Client::new()?,
            config
        })
    }

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

#[cfg(test)]
mod test {
    use super::*;

    fn mock_client<F>(f: F)
    where F: FnOnce(&Client)
    {
        let config = Config {
            repository: "nrc/find-work".to_owned(),
            username: ::TEST_USERNAME.to_owned(),
            token: ::TEST_TOKEN.to_owned(),
            base_path: String::new(),
            port: 0,
        };
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
                assert!(file_names.contains(&data::TABS.to_owned()));
                assert!(file_names.contains(&data::CATEGORIES.to_owned()));
                assert!(file_names.contains(&data::TAB_CATEGORY.to_owned()));
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

    // #[test]
    // fn test_fetch_structural_data() {
    //     let config = Config {
    //         repository: "nrc/find-work".to_owned(),
    //         username: ::TEST_USERNAME.to_owned(),
    //         token: ::TEST_TOKEN.to_owned(),
    //         base_path: String::new(),
    //         port: 0,
    //     };
    //     assert!(fetch_structural_data(&config).is_ok());
    // }
}
