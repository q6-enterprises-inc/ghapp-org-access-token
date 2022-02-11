use std::{
    env,
    error::Error
};

#[derive(Debug)]
pub struct Config {
    pub base_url: String,
    pub app_id: String,
    pub private_key_path: String,
}

impl Config {
    pub fn new () -> Result<Config, Box<dyn Error>>  {
        let base_url = env::var("GITHUB_API_URL")?;
        let app_id = env::var("APP_ID")?;
        let private_key_path = env::var("PRIVATE_KEY_PATH")?;

        Ok(Config {
            base_url,
            app_id,
            private_key_path,
        })
    }
}
