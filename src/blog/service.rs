use anyhow::Result;
use spin_sdk::pg;

use crate::config::get_db_url;

use super::Article;

pub fn all_articles() -> Result<Vec<Article>> {
    let db_url = get_db_url()?;
    let sql = "SELECT * FROM articles";
    let rowset = pg::query(&db_url, sql, &[])?;
    // let column_summary = rowset
    //     .columns
    //     .iter()
    //     .map(format_col)
    //     .collect::<Vec<_>>()
    //     .join(", ");

    // let resp_lines = vec![];
    rowset.rows.iter().map(Article::try_from).collect()
}

// fn format_col(col: &pg::Column) -> String {
//     format!("{}:{:?}", col.name, col.data_type)
// }
