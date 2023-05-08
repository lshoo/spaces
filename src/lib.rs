pub mod blog;
pub mod config;

use anyhow::Result;

use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

use crate::blog::pg_backend_pid;

/// A simple Spin HTTP component.
#[http_component]
fn process(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let mut router = Router::new();

    router.get("/api/article/:id", blog::get_article);
    router.get("/api/article", blog::list_article);
    router.post("api/article", blog::save);
    router.get("/api/pg_pid", pg_backend_pid);
    router.any("/*", echo_wildcard);

    router.handle(req)
}

fn echo_wildcard(_req: Request, params: Params) -> Result<Response> {
    let capture = params.wildcard().unwrap_or_default().to_string();
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(capture.into()))?)
}
