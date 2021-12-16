#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
#[macro_use]
extern crate diesel;
mod controllers;
mod db;
mod models;
mod schema;

// For reference: https://rocket.rs/v0.5-rc/guide/requests/

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/api",
            routes![controllers::users::index, controllers::users::create_user],
        )
        .mount("/", FileServer::from(relative!("static")))
}
