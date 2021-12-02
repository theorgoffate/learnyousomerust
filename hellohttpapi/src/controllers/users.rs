use serde_json::json;
use crate::models::user::User;

#[get("/users")]
pub fn index() -> String {
    let users = vec![User { name: "foobar".into() }];
    json!(users).to_string()
}