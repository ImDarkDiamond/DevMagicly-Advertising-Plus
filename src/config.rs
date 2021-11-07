#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            token: std::env::var("TOKEN").expect("No token provided")
        }
    }
}