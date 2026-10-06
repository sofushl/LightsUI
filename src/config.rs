use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub ip: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            ip: env::var("IP").expect("Missing IP"),
            port: env::var("PORT").expect("Missing PORT"),
        }
    }
}
