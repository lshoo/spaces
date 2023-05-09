use anyhow::Result;

use spin_sdk::http::{Params, Request, Response};

use super::{all_articles, get_article_by_id, pg_pid, save_articles, ArticleRequest};

fn build_response(code: u16, body: impl Into<String>) -> Result<Response> {
    // let body = serde_json::to_string(&body)?;
    // Ok(http::Response::builder()
    //     .status(code)
    //     .body(Some(body.into().into()))?)
    build_response_option(code, Some(body))
}

fn build_response_option(code: u16, body: Option<impl Into<String>>) -> Result<Response> {
    Ok(http::Response::builder()
        .status(code)
        .body(body.map(|b| b.into().into()))?)
}

pub fn handle_get(_req: Request, params: Params) -> Result<Response> {
    let article_id = params.get("id").expect("missing id").parse::<i64>()?;
    let body = serde_json::to_string(&get_article_by_id(article_id)?)?;
    build_response(http::StatusCode::OK.as_u16(), body)
}

pub fn handle_list(_req: Request, _param: Params) -> Result<Response> {
    build_response(200, serde_json::to_string(&all_articles()?)?)
}

pub fn handle_save(req: Request, _param: Params) -> Result<Response> {
    let body: Option<String> = req
        .body()
        .as_ref()
        .map(|b| String::from_utf8(b.to_vec()).unwrap());
    let to_article = |s: String| serde_json::from_str::<ArticleRequest>(&s);

    let art_req = body.map(to_article).transpose()?;
    match art_req {
        Some(art) => {
            let res = save_articles(art)?;
            build_response(200, res.to_string())
        }
        None => build_response(200, "empty body"),
    }
}

pub fn pg_backend_pid(_req: Request, _params: Params) -> Result<Response> {
    let pid = pg_pid()?;
    build_response(200, pid.to_string())
}

pub fn not_found() -> Result<Response> {
    // build_response(404, "not found")
    spin_sdk::http::not_found()
}
