use serde::Deserialize;
//Deserialize
#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub port: u16,
    pub debug: bool,
}