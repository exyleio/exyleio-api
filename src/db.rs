use rocket::fairing::AdHoc;
use rocket::State;

const REDIS_ADDRESS: &'static str = "redis://0.0.0.0:6379";

pub struct DB {
    pub client: redis::Client,
}

pub fn stage() -> AdHoc {
    let client = redis::Client::open(REDIS_ADDRESS).unwrap();

    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket.manage(DB { client })
    })
}

pub fn redis_cmd<T: redis::FromRedisValue>(db: &State<DB>, command: &str) -> Option<T> {
    let mut conn = db.client.get_connection().unwrap();
    return redis::cmd(command).query(&mut conn).unwrap();
}
