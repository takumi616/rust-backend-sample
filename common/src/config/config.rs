use std::env;

pub struct Config {
    pub app_port: String
}

impl Config {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(Self {
            app_port: env::var("CONTAINER_APP_PORT")?,
        })
    }
}

