use crate::db::DB;
use rocket::{Route, State};

#[get("/ping")]
fn ping(db: &State<DB>) -> Option<String> {
    let mut conn = db.client.get_connection().unwrap();
    let reply = redis::cmd("PING").query(&mut conn).unwrap();

    Some(reply)
}

pub fn routes() -> Vec<Route> {
    routes![ping]
}
