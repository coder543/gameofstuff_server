#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use rocket::request::Form;
use rocket_contrib::Json;

mod db;
use db::DbConn;
use db::game;
use db::player;
use db::topic;

#[get("/get_topic/<game_id>/<category>")]
pub fn get_topic(
    conn: DbConn,
    game_id: i64,
    category: String,
) -> Result<Json<topic::Topic>, &'static str> {

    let topics = topic::get_topics(&conn, category);
    let used_topics = topic::get_used_topic_ids(&conn, game_id);

    let topic = topic::get_unused_topic(topics, used_topics)?;
    Ok(Json(topic))
}

#[post("/get_players", data = "<game>")]
fn get_players(
    conn: DbConn,
    game: Form<game::GameForm>,
) -> Result<Json<Vec<player::Player>>, &'static str> {

    let game_name = &game.get().name;
    Ok(Json(vec![
        player::Player {
            id: 0,
            name: "Test".into(),
            score: 10,
        },
        player::Player {
            id: 1,
            name: "Other".into(),
            score: 7,
        },
    ]))
}

fn main() {
    let pool = db::get_pool();
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![get_players, get_topic])
        .launch();
}
