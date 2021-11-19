#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
mod api;
mod api2;

// For reference: https://rocket.rs/v0.5-rc/guide/requests/

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/api2", routes![api2::index, api2::test1, api2::test2])
        .mount("/api", routes![api::index, api::introduce])
        .mount("/", FileServer::from(relative!("static")))
}
