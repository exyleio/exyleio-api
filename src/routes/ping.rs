use crate::db::{redis_cmd, DB};
use rocket::State;
use rocket_okapi::openapi;

#[openapi()]
#[get("/ping")]
pub fn ping(db: &State<DB>) -> Option<String> {
    redis_cmd(db, "PING")
}
