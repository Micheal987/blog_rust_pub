use serde::Deserialize;
use self::super::app_config::App;
use self::super::db_config::Database;
use self::super::xdb_filepath_config::Ip2region;
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub database: Database,
    pub ip2region:Ip2region,
}
