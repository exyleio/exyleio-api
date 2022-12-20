use rocket::fairing::AdHoc;

const REDIS_ADDRESS: &'static str = "redis://localhost:6379";

pub struct DB {
    pub client: redis::Client,
}

pub fn stage() -> AdHoc {
    let client = redis::Client::open(REDIS_ADDRESS).unwrap();

    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket.manage(DB { client })
    })
}
