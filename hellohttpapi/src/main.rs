#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
mod models;
mod controllers;

// For reference: https://rocket.rs/v0.5-rc/guide/requests/

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/api", routes![controllers::users::index])
        .mount("/", FileServer::from(relative!("static")))
}
