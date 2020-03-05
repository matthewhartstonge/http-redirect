use hyper::header::HOST;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Result, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

// redirect pulls out the requested URI and sends back a redirect.
async fn redirect(req: Request<Body>) -> Result<Response<Body>> {
    // extract the host (and port) from the request...
    let host = match req.headers().get(HOST) {
        None => "",
        Some(header) => {
            let ret = match header.to_str() {
                Ok(h) => h,
                Err(_) => "",
            };
            ret
        }
    };

    // extract the path and query from the request...
    let uri = req.uri().to_string();

    // Build the redirect uri string enforcing an HTTPS scheme...
    let redirect_uri = format!("https://{}{}", host, uri);

    // Build the response...
    let response = Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY) // HTTP 301
        .header("location", redirect_uri)
        .body("".into())
        .unwrap();

    // Respond with the HTTPS redirect!
    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // A `Service` is needed for every connection, so this
    // creates one from our `redirect` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(redirect))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("http-redirect serving from http://127.0.0.1:8080");

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
