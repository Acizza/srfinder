#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod airport_data;
mod path;
mod server_route;

use anyhow::{Context, Result};
use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;

#[rocket::main]
async fn main() -> Result<()> {
    let config = {
        let env = Environment::active().context("failed to get Rocket config")?;

        Config::build(env)
            .workers(1)
            .finalize()
            .context("failed to build Rocket config")?
    };

    airport_data::ensure_updated().context("failed to update airport data")?;

    println!("loading airport data..");
    let airports = airport_data::Airport::load_all().context("failed to load airport data")?;
    println!("finished loading airport data");

    rocket::custom(config)
        .manage(airports)
        .mount("/", StaticFiles::from("frontend/public/"))
        .mount("/api", routes![server_route::search_routes::search_routes])
        .launch()
        .await
        .context("failed to initialize Rocket")
}
