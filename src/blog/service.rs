use anyhow::Result;
use spin_sdk::pg::{self, Decode, ParameterValue};

use crate::config::get_db_url;

use super::{Article, CreateArticleRequest};

pub(crate) fn all_articles() -> Result<Vec<Article>> {
    let db_url = get_db_url()?;
    let sql = "SELECT * FROM articles";
    let rowset = pg::query(&db_url, sql, &[])?;

    rowset.rows.iter().map(Article::try_from).collect()
}

pub(crate) fn get_article_by_id(id: i64) -> Result<Option<Article>> {
    println!("Query article by id: {}", id);

    let db_url = get_db_url()?;

    let sql = "SELECT * FROM public.articles WHERE id = $1";
    let params = vec![ParameterValue::Int64(id)];
    let rowset = pg::query(&db_url, sql, &params)?;

    let resp: Option<Article> = rowset.rows.first().map(|row| row.try_into()).transpose()?;

    Ok(resp)
}

pub(crate) fn save_articles(art: CreateArticleRequest) -> Result<u64> {
    let db_url = get_db_url()?;
    let sql = "INSERT INTO public.articles (title, content, author, coauthor, category) VALUES ($1, $2, $3, $4, $5) RETURNING id";

    let coauthor = &art.coauthor.as_deref();
    let coauthor_param = coauthor
        .map(ParameterValue::Str)
        .unwrap_or(ParameterValue::DbNull);
    let category_param = art
        .category
        .as_deref()
        .map(ParameterValue::Str)
        .unwrap_or(ParameterValue::DbNull);
    let params = vec![
        ParameterValue::Str(&art.title),
        ParameterValue::Str(&art.content),
        ParameterValue::Str(&art.author),
        coauthor_param,
        category_param,
    ];
    let nrow_executed = pg::execute(&db_url, sql, &params)?;

    Ok(nrow_executed)
}

pub(crate) fn pg_pid() -> Result<i32> {
    let db_url = get_db_url()?;
    let sql = "SELECT pg_backend_pid()";

    let get_pid = || {
        let rowset = pg::query(&db_url, sql, &[])?;
        let row = &rowset.rows[0];

        i32::decode(&row[0])
    };

    Ok(get_pid()?)
}
