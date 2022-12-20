use rocket::Route;

mod ping;

pub fn routes() -> Vec<Route> {
    let mut routes = vec![];

    routes.append(&mut ping::routes());

    routes
}
