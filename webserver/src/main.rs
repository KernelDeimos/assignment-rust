extern create hyper;

use hyper::{Body, Request, Response, Server}
use hyper::rt::Future
use hyper::service::service_fn_ok;

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, HTTP Client!"))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3123).into();

    let new_svc = || {
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr)
        .server(new_svc)
        .map_err(|e| eprintln!("error: {}", e));
    
    hyper::rt::run(server);
    println!("Hello, world!");
}
