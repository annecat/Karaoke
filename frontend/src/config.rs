use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backoffice_url: String,
}

impl Config {
    pub fn load() -> Config {
        // Load environment variables from a .env file
        dotenv::dotenv().ok();
        let env = match env::var("KARAOKE_BACK_URL")
        {
            Ok(content) => content,
            _ => "http://127.0.0.1:8000/api".to_string()
        };
        println!("Backoffice URL: {}", env);
        Config {
            backoffice_url : env
        }
    }
}
