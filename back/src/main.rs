extern crate base64;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod blob;
mod config;
mod data;
mod github;
mod issues;
#[cfg(test)]
mod mock;
mod server;

use blob::Blob;
use config::Config;
use server::ServerData;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
const TEST_USERNAME: &'static str = "nrc";
#[cfg(test)]
const TEST_TOKEN: &'static str = include_str!("../test-token.txt");

// In seconds.
const REFRESH_TIMEOUT: u64 = 60 * 60;


fn main() {
    env_logger::init().unwrap();
    if let Err(e) = run() {
        eprintln!("An error occured: {}", e.0);
    }
}

type Result<T> = std::result::Result<T, WorkErr>;

#[derive(Clone, Debug)]
pub struct WorkErr(String);

impl<T: ToString> From<T> for WorkErr {
    fn from(e: T) -> Self {
        WorkErr(e.to_string())
    }
}

// Run the server.
fn run() -> Result<()> {
    info!("starting");

    let server_data = Arc::new(Mutex::new(init()?));
    let server_data_ref = server_data.clone();
    // Refresh data every hour.
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(REFRESH_TIMEOUT));
            let mut server_data = server_data_ref.lock().unwrap();
            match make_blob(&server_data.config) {
                Ok(blob) => {
                    server_data.blob = blob;
                    server_data.file_cache = HashMap::new();
                }
                Err(e) => {
                    // FIXME we should probably do more to indicate that making the blob failed.
                    eprintln!("Error making blob: {}", e.0);
                }
            }
        }
    });

    server::startup(server_data)?;
    Ok(())
}

// Initialise by reading the config, then fetching data from GitHub.
fn init() -> Result<ServerData> {
    let config = config::read_config()?;
    let blob = make_blob(&config)?;
    Ok(ServerData {
        config,
        blob,
        file_cache: HashMap::new(),
    })
}

// Fetch data from GitHub and lower it into the frontend format.
fn make_blob(config: &Config) -> Result<Blob> {
    let struct_data = data::fetch_structural_data(config)?;
    let issues = issues::fetch_issues(config, &struct_data)?;
    Blob::make(&struct_data, &issues)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        init().unwrap_or_else(|s| panic!("{:?}", s));
    }
}
