use crate::core::db_diesel::establish_connection;
use crate::models::user_models::UserModel;
use actix_web::{post, web};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

fn query_list() {
    use crate::schema::user_models::dsl::*;
    let connection = &mut establish_connection();
    let results = user_models
        .limit(5)
        .select(UserModel::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} user", results.len());
    for user in results {
        println!("{:?}", Some(user.addr));
        println!("-----------\n");
    }
}
#[derive(Deserialize)]
struct Info {
    name: String,
}
#[post("/select")]
pub async fn select(info: web::Query<Info>) -> String {
    query_list();
    format!("Welcome {}!", info.name)
}
