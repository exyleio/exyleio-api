use crate::db::DB;
use crate::util::redis_cmd;
use rocket::State;
use rocket_okapi::openapi;

#[openapi()]
#[get("/ping")]
pub fn ping(db: &State<DB>) -> Option<String> {
    redis_cmd(db, "PING")
}
