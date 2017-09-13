use rocket::request::Form;
use rocket_contrib::Json;

use db::DbConn;
use game::GameForm;

#[derive(Serialize)]
struct Player {
    id: i64,
    name: String,
    score: u32,
}

#[post("/get_players", data = "<game>")]
fn get_players(conn: DbConn, game: Form<GameForm>) -> Result<Json<Vec<Player>>, &'static str> {
    // let game_name = &game.get().name;

    Ok(Json(vec![
        Player {
            id: 0,
            name: "Test".into(),
            score: 10,
        },
        Player {
            id: 1,
            name: "Other".into(),
            score: 7,
        },
    ]))
}