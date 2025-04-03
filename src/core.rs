pub mod config;
pub mod core;
pub mod db_diesel;

use config::main;
pub fn init_config_yaml() {
    main();
}
