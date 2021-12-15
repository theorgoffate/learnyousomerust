use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct User {
    pub name: &'static str
}
