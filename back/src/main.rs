extern crate env_logger;
#[macro_use]
extern crate log;
//extern crate serde;
#[macro_use]
extern crate serde_derive;
//#[macro_use]
//extern crate serde_json;


mod data;
mod github;
mod issues;
mod server;

// # endpoints
// /data - return the data blob
// /static/* - serve a static file
// /* - serve front/out/index.html

// # startup
// read config
// run cron job
// schedule cron job
// start server

// # cron job
// pull data from home repo
// run queries
// build blob

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
}
