use crate::db::DB;
use rocket::State;

pub fn redis_cmd<T: redis::FromRedisValue>(db: &State<DB>, command: &str) -> Option<T> {
    let mut conn = db.client.get_connection().unwrap();
    return redis::cmd(command).query(&mut conn).unwrap();
}
