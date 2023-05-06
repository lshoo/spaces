pub mod blog;
pub mod config;

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::blog::all_articles;

/// A simple Spin HTTP component.
#[http_component]
fn process(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    match req.uri().path() {
        "/all" => all(),
        _ => not_found(),
    }
}

fn build_response(code: u16, body: impl Into<String>) -> Result<Response> {
    // let body = serde_json::to_string(&body)?;
    Ok(http::Response::builder()
        .status(code)
        .body(Some(body.into().into()))?)
}

fn all() -> Result<Response> {
    build_response(200, serde_json::to_string(&all_articles()?)?)
}

fn not_found() -> Result<Response> {
    build_response(404, "not found")
}
