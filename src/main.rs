use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

fn main() {
    let mut p = WebLitener { p: 8081 };
    let q = 3012;
    p.set_port(q);
    p.listener();
}

#[derive(Clone, Copy)]
pub struct WebLitener {
    pub p: u16,
}

impl WebLitener {
    fn set_port(&mut self, port: u16) {
        self.p = port;
    }

    fn listener(&self) {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let addr = SocketAddr::from(([127, 0, 0, 1], self.p));

                /*  let make_svc = make_service_fn(|_conn| {
                let db = Arc::new(Mutex::new(self.p));
                async move {
                  Ok::<_, Infallible>(service_fn(move |r| hello_world(r, db.clone())))
                  }
                }); */

                let make_svc = make_service_fn(|_| {
                    let db = Arc::new(Mutex::new(self.p));
                    async move {
                        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                            let dbx = db.clone();
                            async move { hello_world(req, dbx.clone()).await }
                        }))
                    }
                });

                /* let db = Arc::new(Mutex::new(self.p));
                let make_svc = make_service_fn(|_| {
                    let onion1 = Arc::clone(&db);
                    async move {
                        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                            let onion3 = Arc::clone(&onion1);
                            async move {
                                let onion4 = onion3.clone();
                                hello_world(req, onion4.clone()).await
                            }
                        }))
                    }
                }); */

                let server = Server::bind(&addr).serve(make_svc);
                if let Err(e) = server.await {
                    eprintln!("server error: {}", e);
                }
            })
    }
}

pub async fn read_response_body(res: Request<Body>) -> Result<String, hyper::Error> {
    let bytes = body::to_bytes(res.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8"))
}

async fn hello_world(
    req: Request<Body>,
    port: Arc<Mutex<u16>>,
) -> Result<Response<Body>, Infallible> {
    println!("{}", req.uri().path());
    println!("{:?}", req.headers());
    println!("{}", read_response_body(req).await.unwrap());
    Ok(Response::new(port.lock().unwrap().to_string().into()))
}
