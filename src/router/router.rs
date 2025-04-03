use crate::api::addr_api::addr_query_info_api::{addr};
use crate::api::addr_api::addr_list_api::select;
use actix_web::{App, HttpServer};
use xdb::searcher_init;
use crate::core::config::load_config;
#[tokio::main] // or #[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // 初始化日志
    let _config_result = load_config().await;
    tracing_subscriber::fmt::init();
    let file_path  = _config_result.unwrap().ip2region.xdb_filepath;
    // 初始化IP数据库
    searcher_init(Some(file_path));
    HttpServer::new(|| App::new()
        .service(addr)
        .service(select)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
