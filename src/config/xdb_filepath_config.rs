use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Ip2region {
    pub xdb_filepath: String,
}