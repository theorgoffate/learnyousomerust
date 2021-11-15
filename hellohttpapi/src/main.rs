#[macro_use] extern crate rocket;

use rocket::Build;
// For reference: https://rocket.rs/v0.5-rc/guide/requests/

#[get("/introduce/<name>")]
fn introduce(name: &str) -> String {
    format!("Welcome, {}!", name)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                introduce
            ]
        )
}