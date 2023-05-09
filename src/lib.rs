pub mod blog;
pub mod config;

use anyhow::Result;

use serde::{Deserialize, Serialize};
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

    router.get("/api/article/:id", blog::handle_get);
    router.get("/api/article", blog::handle_list);
    router.post("/api/article", blog::handle_save);
    router.put("/api/article/:id", blog::handle_update);
    router.delete("/api/article/:id", blog::handle_delete);
    router.get("/api/pg_pid", pg_backend_pid);
    router.any("/*", echo_wildcard);

    router.handle(req)
}

fn echo_wildcard(_req: Request, params: Params) -> Result<Response> {
    let capture = params.wildcard().unwrap_or_default().to_string();
    let json = serde_json::to_string(&JsonResponse::ok(capture))?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(json.into()))?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonResponse<T: Serialize> {
    Ok {
        code: u16,
        message: Option<String>,
        data: Option<T>,
    },
    Error {
        code: u16,
        message: Option<String>,
    },
}

impl<T: Serialize> JsonResponse<T> {
    /// ok without any message
    pub fn ok(data: T) -> Self {
        Self::Ok {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    /// ok with message
    pub fn ok_with_message(data: T, message: String) -> Self {
        Self::Ok {
            code: 200,
            message: Some(message),
            data: Some(data),
        }
    }

    /// error without any message
    pub fn error(code: u16) -> Self {
        Self::Error {
            code,
            message: None,
        }
    }

    /// error with message
    pub fn error_with_message(code: u16, message: String) -> Self {
        Self::Error {
            code,
            message: Some(message),
        }
    }
}

pub fn to_json<T: Serialize>(data: JsonResponse<T>) -> Result<String> {
    serde_json::to_string(&data).map_err(anyhow::Error::from)
}
