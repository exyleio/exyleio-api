use crate::db::DB;
use rocket::{Route, State};
use rocket_okapi::{openapi, openapi_get_routes};

#[openapi()]
#[get("/ping")]
pub fn ping(db: &State<DB>) -> Option<String> {
    let mut conn = db.client.get_connection().unwrap();
    let reply = redis::cmd("PING").query(&mut conn).unwrap();

    Some(reply)
}
