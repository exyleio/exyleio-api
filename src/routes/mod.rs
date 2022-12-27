//! Contains all API Endpoints

use rocket::Route;
use rocket_okapi::openapi_get_routes;

mod ping;
mod players;

/// All routes should be registered here.
pub fn routes() -> Vec<Route> {
    openapi_get_routes![ping::ping, players::player::get_player]
}
