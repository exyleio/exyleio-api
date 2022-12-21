use crate::json::Player;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// # Get user
///
/// Returns a player object
#[openapi()]
#[get("/players/<id>")]
pub fn get_player(id: String) -> Option<Json<Player>> {
    Some(Json(Player {
        username: String::from("Testing"),
    }))
}
