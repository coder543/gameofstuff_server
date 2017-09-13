use rocket_contrib::Json;

use db::DbConn;

#[derive(Clone, Debug, Serialize)]
pub struct Topic {
    id: i64,
    topic: String,
}

fn get_used_topic_ids(conn: &DbConn, game_id: i64) -> Vec<i64> {
    conn.query("select topic_id from gameround where game=$1", &[&game_id])
        .unwrap()
        .iter()
        .map(|topic| topic.get(0))
        .collect()
}

fn get_topics(conn: &DbConn, category: String) -> Vec<Topic> {
    conn.query(
        "select id, topic from topic where categories @> array[$1]::text[]",
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
pub fn get_topic(
    conn: DbConn,
    game_id: i64,
    category: String,
) -> Result<Json<Topic>, &'static str> {

    let topics = get_topics(&conn, category);
    let used_topics = get_used_topic_ids(&conn, game_id);

    let topic = get_unused_topic(topics, used_topics)?;
    Ok(Json(topic))
}