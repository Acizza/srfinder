#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod airport_data;
mod api;
mod path;

use airport_data::our_airports::OurAirports;
use airport_data::AirportData;
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

    let mut airports_source = OurAirports::init().context("failed to init OurAirports data")?;

    if airports_source.is_up_to_date() {
        println!("loading OurAirports data..");
    } else {
        println!("updating OurAirports data..");
        airports_source.update().context("update failed")?;
    };

    let airports = airports_source
        .load()
        .context("failed to load OurAirports data")?;

    println!("finished loading OurAirports data");

    rocket::custom(config)
        .manage(airports)
        .mount("/", StaticFiles::from("frontend/public/"))
        .mount("/api", routes![api::search_routes::search_routes])
        .launch()
        .await
        .context("failed to initialize Rocket")
}
