use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref APP_LANGUAGE: String = {
        dotenv::dotenv().ok();
        env::var("APP_LANG").unwrap()
    };
}
