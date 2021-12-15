use diesel::RunQueryDsl;
use serde_json::json;
use crate::models::user::User;
use crate::schema::users::dsl::users;
use crate::db;

#[get("/users")]
pub fn index() -> String {
    let conn = db::establish_connection();
    let found_users: Vec<User> = match users.load::<User>(&conn){
        Some(v) => v,
        Err(e) => {
            println!("error finding users: {}", e);
            vec![]
        }
    };
    json!(found_users).to_string()
}
