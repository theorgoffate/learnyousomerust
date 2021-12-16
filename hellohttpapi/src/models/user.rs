use rocket::serde::{Deserialize, Serialize};
#[derive(PartialEq, Debug, Serialize, Deserialize, Queryable)]

pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub name: Option<String>,
}
