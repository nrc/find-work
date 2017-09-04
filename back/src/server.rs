use config::Config;
use blob::Blob;

use futures::future;
use hyper::{self, Method, StatusCode};
use hyper::server::{Http, Request, Response, Service, NewService};

use std::sync::{Arc, Mutex};


pub struct ServerData {
    pub config: Config,
    pub blob: Blob,
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
        if req.method() == &Method::Get {
            return Route::Unknown;
        }
        match req.path() {
            "/data" => Route::Data,
            path if path.starts_with("/static/") => {
                Route::Static(path["/static/".len()..].to_owned())
            }
            _ => Route::Index,
        }
    }
}

impl Service for WorkService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut res = Response::new();
        match WorkService::route(&req) {
            Route::Index => {}
            Route::Data => {}
            Route::Static(_) => {}
            Route::Unknown => {
                res.set_status(StatusCode::NotFound);
            }
        }

        future::ok(res)
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
