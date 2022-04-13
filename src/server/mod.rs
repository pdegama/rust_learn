use std::convert::Infallible;
use std::fs;
use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::io::Write;

pub mod webres;

pub async fn run_server(ip_add: [u8; 4], port: u16) {

    async fn wer_res(_req: Request<Body>, c: Arc<Mutex<i32>>) -> Result<Response<Body>, Infallible> {
        //access variable to thread
        *c.lock().unwrap() += 1;
        //println!("{:?}", c);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("/home/parthka/IdeaProjects/rust_learn/file.txt")
            .unwrap();

        file.write_all(b"to append\n");
        Ok(Response::new(webres::r1().into()))
    }

    // We'll bind to <ip_add>:<port>
    let addr = SocketAddr::from((ip_add, port));

    // A `Service` is needed for every connection, so this
    // creates one from our `wer_res` function.
    let make_svc = make_service_fn(|_conn| async {
        let db = Arc::new(Mutex::new(49));
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(move |req| wer_res(req, db.clone())))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}