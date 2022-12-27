use crate::json::{Player, Tag};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Returns a [player](crate::json::Player) object with the given id
#[openapi()]
#[get("/players/<id>")]
pub fn get_player(id: String) -> Option<Json<Player>> {
    Some(Json(Player {
        id,
        username: String::from("Testing"),
        tags: vec![Tag::POMP],
    }))
}
