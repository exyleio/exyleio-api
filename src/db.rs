//! Handles everything related to Redis DB
//!
//! Redis DB Structure:
//!
//! - [Data types](https://redis.io/docs/data-types)
//!
//! | Key                                          | Type       | Field/Member                  | Value/Score                                    |
//! | -------------------------------------------: | :--------: | :---------------------------: | :--------------------------------------------: |
//! | `rank:<game-mode>`                           | Sorted Set | player id                     | ranking criteria                               |
//! | `rank:<game-mode>:days-number-one`           | Sorted Set | player id                     | total number of days player was in the #1 rank |
//! | `rank:<game-mode>:longest-number-one-streak` | Sorted Set | player id                     | player's longest #1 streak                     |
//! | `rank:xp`                                    | Sorted Set | player id                     | Player XP                                      |
//! | `player:<player-id>:items`                   | Set        | item id                       | -                                              |
//! | `player:<id>`                                | Hash       | [Player](crate::json::Player) | in-exhaustive player data                      |

use rocket::fairing::AdHoc;
use rocket::State;

const REDIS_ADDRESS: &'static str = "redis://redis:6379";

/// Database Struct that gets passed when there's a request
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
