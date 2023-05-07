
use std::env::VarError;

use spin_sdk::key_value::{Store, Error};

pub const DB_URL_ENV: &str = "DB_URL";

/// Get db url from command
pub fn get_db_url() -> Result<String, Error> {
    let store = Store::open_default()?;
    store.get(DB_URL_ENV)
        .map(|v| String::from_utf8(v).unwrap())
}

pub fn get_db_url_env() -> Result<String, VarError> {
    // let url = std::env::var(DB_URL_ENV).expect("Please set DB_URL");
    // println!("{}", url);
    // Ok(url)
    std::env::var(DB_URL_ENV)
}
