#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use postgres::Connection;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// An alias to the type for a pool of postgresql connections.
type Pool = r2d2::Pool<PostgresConnectionManager>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<PostgresConnectionManager>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using &DbConn as &Connection.
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use rocket::request::Form;
use rocket_contrib::Json;

#[derive(FromForm)]
struct GameForm {
    name: String,
}

#[derive(Serialize)]
struct Player {
    id: i64,
    name: String,
    score: u32,
}

#[derive(Clone, Debug, Serialize)]
struct Topic {
    id: i64,
    topic: String,
}

fn get_used_topic_ids(conn: &DbConn, game_id: i64) -> Vec<i64> {
    conn.query("select topic from gameround where game=$1", &[&game_id])
        .unwrap()
        .iter()
        .map(|topic| topic.get(0))
        .collect()
}

fn get_topics(conn: &DbConn, category: String) -> Vec<Topic> {
    conn.query(
        "select id, topic from topic where category=$1",
        &[&category],
    ).unwrap()
        .iter()
        .map(|topic| {
            Topic {
                id: topic.get(0),
                topic: topic.get(1),
            }
        })
        .collect()
}

fn get_unused_topic(topics: Vec<Topic>, used_topic_ids: Vec<i64>) -> Result<Topic, &'static str> {
    let topic = topics
        .iter()
        .filter(|topic| !used_topic_ids.contains(&topic.id))
        .next()
        .ok_or("no more topics")?;
    Ok(topic.clone())
}

#[get("/get_topic/<game_id>/<category>")]
fn get_topic(conn: DbConn, game_id: i64, category: String) -> Result<Json<Topic>, &'static str> {
    let used_topics = get_used_topic_ids(&conn, game_id);
    let topics = get_topics(&conn, category);
    let topic = get_unused_topic(topics, used_topics)?;

    Ok(Json(topic))
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

fn main() {
    let config = r2d2::Config::default();
    let manager =
        PostgresConnectionManager::new("postgres://coder:coder@localhost/coder", TlsMode::None)
            .unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![get_players, get_topic])
        .launch();
}
