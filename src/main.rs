use hyper::service::{make_service_fn, service_fn};
use hyper::{ext, Body, Request, Response, Result, Server};
use std::convert::Infallible;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = Response::new(Body::from("Hello from Rust-Server!"));
    Ok(response)
}



#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1],8080).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started at http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Error starting the server! {}", e);
    }

}
