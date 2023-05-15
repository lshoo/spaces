use anyhow::Result;

use spin_sdk::http::{Params, Request, Response};

use crate::{to_json, JsonResponse};

use super::{
    all_articles, delete_article_by_id, get_article_by_id, pg_pid, save_articles,
    update_article_by_id, CreateArticleRequest, UpdateArticleRequest,
};

fn build_response(code: u16, body: impl Into<String>) -> Result<Response> {
    build_response_option(code, Some(body))
}

fn build_response_option(code: u16, body: Option<impl Into<String>>) -> Result<Response> {
    Ok(http::Response::builder()
        .status(code)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Max-Age", "86400")
        .body(body.map(|b| b.into().into()))?)
}

pub fn handle_get(_req: Request, params: Params) -> Result<Response> {
    let article_id = params.get("id").expect("missing id").parse::<i64>()?;
    let article = get_article_by_id(article_id)?;
    let body = article.map(|a| to_json(JsonResponse::ok(a))).transpose()?;
    build_response_option(http::StatusCode::OK.as_u16(), body)
}

pub fn handle_list(_req: Request, _params: Params) -> Result<Response> {
    let body = to_json(JsonResponse::ok(all_articles()?))?;
    build_response(200, body)
}

pub fn handle_save(req: Request, _params: Params) -> Result<Response> {
    let article: CreateArticleRequest = req.body().clone().try_into()?;

    let res = save_articles(article)?;
    let body = to_json(JsonResponse::ok(res))?;
    build_response(200, body)
}

pub fn handle_update(req: Request, params: Params) -> Result<Response> {
    let article_id = params.get("id").expect("missing id").parse::<i64>()?;
    let article: UpdateArticleRequest = req.body().clone().try_into()?;

    let res = update_article_by_id(article_id, article)?;
    let body = to_json(JsonResponse::ok(res))?;
    build_response(200, body)
}

pub fn handle_delete(_req: Request, params: Params) -> Result<Response> {
    let article_id = params.get("id").expect("missing id").parse::<i64>()?;
    let res = delete_article_by_id(article_id)?;
    let body = to_json(JsonResponse::ok(res))?;
    build_response(200, body)
}

pub fn pg_backend_pid(_req: Request, _params: Params) -> Result<Response> {
    let pid = pg_pid()?;
    let body = to_json(JsonResponse::ok(pid))?;
    build_response(200, body)
}

pub fn not_found() -> Result<Response> {
    // build_response(404, "not found")
    spin_sdk::http::not_found()
}
