// src/models.rs
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::user_models;
#[derive(Queryable, Debug, Serialize, Deserialize,Selectable)]
pub struct UserModel {
    pub id: i32,                    // Int4 → i32
    pub name: String,               // Varchar → String (非空)
    pub avatar: Option<String>,     // Nullable<Varchar>
    pub email: Option<String>,      // Nullable<Varchar>
    pub tel: Option<String>,        // Nullable<Varchar>
    pub addr: Option<String>,       // Nullable<Varchar>
    pub link: Option<String>,       // Nullable<Varchar>
    pub sign: Option<String>,       // Nullable<Text>
    pub integral: Option<i32>,      // Nullable<Int4>
    pub ip: Option<String>,         // Nullable<Varchar>
    pub role: String,               // Varchar → String (非空)
    pub sign_status: Option<bool>,  // Nullable<Bool>
    pub created_at: Option<NaiveDateTime>, // Nullable<Timestamp>
    pub updated_at: Option<NaiveDateTime>,
}

// 插入模型（可选）
// 插入模型（处理可空字段）
#[derive(Insertable)]
#[diesel(table_name = user_models)]
pub struct NewUser {
    pub name: String,
    pub role: String,
    pub email: Option<String>,
    pub tel: Option<String>,
    pub avatar: Option<String>,
    pub addr: Option<String>,
    pub link: Option<String>,
    pub sign: Option<String>,
    pub integral: Option<i32>,
    pub ip: Option<String>,
    pub sign_status: Option<bool>,
}