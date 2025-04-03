mod api;
mod config;
mod core;
mod global;
mod models;
mod router;
mod schema;

use self::core::init_config_yaml;
use self::router::init_router;
fn main() {
    init_config_yaml();
    init_router()
}
