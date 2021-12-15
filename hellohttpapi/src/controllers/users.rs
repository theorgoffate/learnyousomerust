use diesel::RunQueryDsl;
use serde_json::json;
use crate::models::user::User;
use crate::schema;
use crate::db;

#[get("/users")]
pub fn index() -> String {
    let conn = db::establish_connection();
    let found_users = match schema::users::dsl::users.load::<User>(&conn){
        Ok(v) => v,
        Err(e) => {
            println!("error finding users: {}", e);
            vec![]
        }
    };
    json!(found_users).to_string()
}
