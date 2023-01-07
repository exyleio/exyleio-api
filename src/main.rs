use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::{
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig, Theme, UiConfig},
    settings::UrlObject,
};

#[macro_use]
extern crate rocket;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

#[cfg(test)]
mod tests;

mod db;
mod json;
mod routes;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .attach(cors)
        .attach(db::stage())
        .mount("/", routes::routes())
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
