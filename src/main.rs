#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

mod db;
mod game;

mod topic;
use topic::*;

mod player;
use player::*;

fn main() {
    let pool = db::get_pool();
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![get_players, get_topic])
        .launch();
}
