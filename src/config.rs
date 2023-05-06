pub const DB_URL_ENV: &str = "DB_URL";

pub fn get_db_url() -> String {
    let url = std::env::var(DB_URL_ENV).expect("Please set DB_URL");
    println!("{}", url);
    url
}
