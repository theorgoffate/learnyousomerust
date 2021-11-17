#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
mod api;

// For reference: https://rocket.rs/v0.5-rc/guide/requests/

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/api", routes![api::index, api::introduce])
        .mount("/", FileServer::from(relative!("static")))
}
