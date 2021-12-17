use crate::db;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::{insert_into, RunQueryDsl};
use rocket::serde::{json::serde_json::json, json::Json};

#[get("/users")]
pub fn index() -> String {
    let conn = db::establish_connection();
    let found_users = match users.load::<User>(&conn) {
        Ok(v) => v,
        Err(e) => {
            println!("error finding users: {}", e);
            vec![]
        }
    };
    json!(found_users).to_string()
}

#[post("/users", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUser>) -> String {
    let conn = db::establish_connection();
    match insert_into(users).values(&new_user.into_inner()).get_result::<User>(&conn) {
        Ok(user) => json!(user).to_string(),
        Err(err) => {
            println!("error creating user: {}", err);
            format!("error creating user")
        }
    }
}
