use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}