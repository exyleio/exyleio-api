use crate::db::{redis_cmd, DB};
use rocket::State;
use rocket_okapi::openapi;

/// Pings the redis database.
/// Should return "PONG" if everything's working properly.
#[openapi()]
#[get("/ping")]
pub fn ping(db: &State<DB>) -> Option<String> {
    redis_cmd(db, "PING")
}
