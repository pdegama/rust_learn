use std::convert::Infallible;
use std::net::{SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

pub mod webres;

pub async fn run_server(ip_add: [u8; 4], port: u16) {
    let user_name = 42;

    async fn wer_res(_req: Request<Body>, c: i32) -> Result<Response<Body>, Infallible> {
        println!("{}", c);
        Ok(Response::new(webres::r1().into()))
    }

    // We'll bind to <ip_add>:<port>
    let addr = SocketAddr::from((ip_add, port));

    // A `Service` is needed for every connection, so this
    // creates one from our `wer_res` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(move |req| wer_res(req, 5465)))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}