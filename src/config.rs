use spin_sdk::key_value::{Error, Store};

pub const DB_URL_ENV: &str = "DB_URL";

/// Get db url from command
pub fn get_db_url() -> Result<String, Error> {
    let store = Store::open_default()?;
    store.get(DB_URL_ENV).map(|v| String::from_utf8(v).unwrap())
}
