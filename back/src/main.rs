extern crate base64;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod data;
mod github;
mod issues;
mod server;

#[cfg(test)]
const TEST_USERNAME: &'static str = "nrc";
#[cfg(test)]
const TEST_TOKEN: &'static str = include_str!("../test-token.txt");

// # endpoints
// /data - return the data blob
// /static/* - serve a static file
// /* - serve front/out/index.html

// # startup
// run cron job
// schedule cron job
// start server

// # cron job
// pull data from home repo
// run queries
// build blob

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

fn run() -> Result<()> {
    info!("starting");

    init()?;

    Ok(())
}

fn init() -> Result<()> {
    let config = data::read_config()?;
    Err(::WorkErr("TODO".to_owned()))
}
