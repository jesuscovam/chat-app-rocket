#[macro_use]
extern crate rocket;
pub mod server_state;

use rocket::{
    fs::{relative, FileServer},
    tokio::sync::broadcast::channel,
};
use server_state::{events, post, Message};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}
