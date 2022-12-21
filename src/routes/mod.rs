use rocket::Route;
use rocket_okapi::openapi_get_routes;

mod ping;
mod players;

pub fn routes() -> Vec<Route> {
    // all routes should be listed here
    openapi_get_routes![ping::ping, players::player::get_player]
}
