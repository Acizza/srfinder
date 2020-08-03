#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod airport_data;
mod path;
mod server_route;

use anyhow::Result;
use anyhow::{anyhow, Context};
use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(err) = airport_data::ensure_updated() {
        return Err(anyhow!("airport data update failed: {}", err));
    }

    println!("loading airport data..");

    let airports = match airport_data::Airport::load_all() {
        Ok(airports) => airports,
        Err(err) => return Err(anyhow!("error loading airport data: {}", err)),
    };

    println!("finished loading airport data..");

    let config = Config::build(Environment::active().unwrap())
        .workers(1)
        .finalize()
        .context("failed to build Rocket config")?;

    rocket::custom(config)
        .manage(airports)
        .mount("/", StaticFiles::from("frontend/dist/"))
        .mount("/api", routes![server_route::search_routes::search_routes])
        .launch()
        .await
        .with_context(|| anyhow!("failed to initialize Rocket"))
}
