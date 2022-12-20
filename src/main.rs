#[macro_use]
extern crate rocket;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

#[cfg(test)]
mod tests;

mod db;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", routes::routes())
}
