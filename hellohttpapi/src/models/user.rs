use rocket::serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(PartialEq, Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: Option<String>
}
