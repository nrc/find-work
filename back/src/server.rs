use config::Config;
use blob::Blob;

use futures::future;
use hyper::{self, Method, StatusCode};
use hyper::header::{ContentType};
use hyper::server::{Http, Request, Response, Service, NewService};
use serde_json;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};


pub struct ServerData {
    pub config: Config,
    pub blob: Blob,
    pub file_cache: HashMap<PathBuf, Vec<u8>>,
}

pub fn startup(data: Arc<Mutex<ServerData>>) -> ::Result<()> {
    let addr = {
        let data = data.lock().unwrap();
        data.config.addr.clone()
    };
    println!("starting up on http://{}", addr);
    let addr = addr.parse()?;
    let service = WorkService { data };
    let server = Http::new().bind(&addr, service)?;
    server.run()?;
    Ok(())
}

#[derive(Clone)]
struct WorkService {
    data: Arc<Mutex<ServerData>>,
}

impl WorkService {
    fn route(req: &Request) -> Route {
        if req.method() != &Method::Get {
            return Route::Unknown;
        }
        match req.path() {
            "/data" => Route::Data,
            path if path.starts_with("/static") => {
                Route::Static(path["/static".len()..].to_owned())
            }
            _ => Route::Index,
        }
    }

    fn load_file(&self, path: &Path) -> ::Result<Vec<u8>> {
        Ok(vec![])
        // TODO
    }
}

impl Service for WorkService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<future::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut res = Response::new();
        match WorkService::route(&req) {
            Route::Index => {
                // TODO char encoding header
                res = res.with_header(ContentType::html());
                // TODO
            }
            Route::Static(_) => {
                // TODO file might not be text, look at extension
                res = res.with_header(ContentType::plaintext());
                // TODO
            }
            Route::Data => {
                // TODO don't block
                let blob = {
                    let data = self.data.lock().unwrap();
                    match serde_json::to_vec(&data.blob) {
                        Ok(blob) => blob,
                        // FIXME errors are a pain to make work, it seems
                        Err(_e) => return Box::new(future::err(hyper::Error::Method)),
                    }
                };
                res = res.with_header(ContentType::json())
                         .with_body(blob);
            }
            Route::Unknown => {
                // TODO char encoding header, message
                res.set_status(StatusCode::NotFound);
            }
        }

        Box::new(future::ok(res))
    }
}

impl NewService for WorkService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = WorkService;

    fn new_service(&self) -> Result<Self::Instance, ::std::io::Error> {
        Ok(self.clone())
    }
}

enum Route {
    Data,
    Index,
    Static(String),
    Unknown,
}
