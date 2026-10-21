use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub picoip: String,
    pub hostip: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            picoip: env::var("PICOIP").expect("Missing PICOIP"),
            hostip: env::var("HOSTIP").expect("Missing HOSTIP"),
            port: env::var("PORT").expect("Missing PORT"),
        }
    }
}
