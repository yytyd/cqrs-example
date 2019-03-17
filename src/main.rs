#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod read_schema;
pub mod users;
pub mod write_schema;

use users::infrastructure::api::user_routes;

fn main() {
    rocket::ignite().mount("/users/c/", user_routes()).launch();
}
