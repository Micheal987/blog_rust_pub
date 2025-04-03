use actix_web::{Responder, post, web};
use diesel::{PgConnection, RunQueryDsl, Table};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use xdb::search_by_ip;

#[derive(Deserialize)]
struct Address {
    ip: String,
}
// 定义返回的JSON数据结构
#[derive(Serialize)]
struct IpInfo {
    region: String,
    country: String,
    province: String,
    city: String,
    isp: String,
}

#[post("/addr")]
async fn addr(info: web::Json<Address>) -> impl Responder {
    // 解析IPv4地址
    let ipv4 = match info.ip.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(_) => {
            return web::Json(IpInfo {
                region: "Invalid IP address".to_string(),
                country: "".to_string(),
                province: "".to_string(),
                city: "".to_string(),
                isp: "".to_string(),
            });
        }
    };

    // 转换为u32（网络字节序）
    let ip_num = u32::from_be_bytes(ipv4.octets());

    // 查询IP信息
    let region_str = match search_by_ip(ip_num) {
        Ok(info) => info,
        Err(e) => {
            return web::Json(IpInfo {
                region: format!("Search error: {}", e),
                country: "".to_string(),
                province: "".to_string(),
                city: "".to_string(),
                isp: "".to_string(),
            });
        }
    };
    //入库
    use crate::core::db_diesel::establish_connection;
    let mut conn = establish_connection();
    create_user_model(&mut conn, region_str.to_string());
    // 解析返回的字符串
    let parts: Vec<&str> = region_str.split('|').collect();
    web::Json(IpInfo {
        region: region_str.clone(),
        country: parts.get(0).unwrap_or(&"").to_string(),
        province: parts.get(2).unwrap_or(&"").to_string(),
        city: parts.get(3).unwrap_or(&"").to_string(),
        isp: parts.get(4).unwrap_or(&"").to_string(),
    })
}

use crate::models::user_models::{NewUser, UserModel};
pub fn create_user_model(conn: &mut PgConnection, addrs: String) -> UserModel {
    use crate::schema::user_models::dsl::*;

    let new_post = NewUser {
        name: "admin".to_string(),
        role: "user".to_string(),
        email: None,
        tel: None,
        avatar: None,
        addr: Some(addrs),
        link: None,
        sign: None,
        integral: None,
        ip: None,
        sign_status: None,
    };

    diesel::insert_into(user_models)
        .values(&new_post)
        .returning(user_models::all_columns()) // ✅ 关键修改点
        .get_result(conn)
        .expect("Error saving user")
}
