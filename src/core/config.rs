use tokio::task;
use crate::config::config::{AppConfig};
pub async fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let contents = tokio::fs::read_to_string("src/setting.yaml").await?;
    // 将反序列化放到阻塞池
    let config = task::spawn_blocking(move|| {
        serde_yaml::from_str::<AppConfig>(&contents)
    }).await??;  // 注意双重错误处理
    Ok(config)
}
#[tokio::main]
pub async fn main() {
    let config_result = load_config().await;

    match config_result {
        Ok(config) => {
            println!("Loaded config:");
            println!("{:#?}", config.app.name);
            println!("{:#?}", config.app.debug);
            println!("{:#?}", config.app.port);
            println!("{:#?}", config.database.url);
            println!("{:#?}", config.database.pool_size);
            println!("{:#?}", config.ip2region.xdb_filepath);
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    }
}
