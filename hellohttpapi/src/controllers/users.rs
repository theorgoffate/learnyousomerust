use crate::db;
use crate::models::user::User;
use crate::schema;
use diesel::RunQueryDsl;
use rocket::serde::{json::serde_json::json, json::Json};

#[get("/users")]
pub fn index() -> String {
    let conn = db::establish_connection();
    let found_users = match schema::users::dsl::users.load::<User>(&conn) {
        Ok(v) => v,
        Err(e) => {
            println!("error finding users: {}", e);
            vec![]
        }
    };
    json!(found_users).to_string()
}

#[post("/users", data = "<new_user>")]
pub fn create_user(new_user: Json<User>) -> String {
    json!(&*new_user).to_string()
}
