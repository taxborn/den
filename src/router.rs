use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{Method, Request, Response, StatusCode};

type BodyBytes = BoxBody<Bytes, hyper::Error>;

pub async fn route(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BodyBytes>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full("Hello, world!"))),
        (method, path) => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;

            eprintln!("[HTTP 404] on {method} {path}");

            Ok(not_found)
        }
    }
}

fn empty() -> BodyBytes {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BodyBytes {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
