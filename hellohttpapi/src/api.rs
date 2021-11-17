#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/introduce/<name>")]
pub fn introduce(name: &str) -> String {
    format!("Welcome, {}!", name)
}
