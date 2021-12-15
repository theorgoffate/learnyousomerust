use serde::Serialize;
#[derive(PartialEq, Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: Option<String>
}
