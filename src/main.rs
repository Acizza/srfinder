#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate time;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

#[macro_use] mod util;
mod airport;
mod filter_form;

use airport::Airport;
use airport::data::{Country, AirportData};
use airport::route::{self, Route};
use filter_form::DataForm;
use rocket_contrib::{Template, Json};
use rocket::{Rocket, State};
use rocket::response::NamedFile;
use rocket::request::LenientForm;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use time::PreciseTime;
use util::ToEnum;

error_chain! {
    links {
        DataFiles(airport::data::Error, airport::data::ErrorKind);
        Route(airport::route::Error, airport::route::ErrorKind);
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &HashMap::<String, String>::new())
}

#[post("/filter", data = "<form>")]
fn filter<'a>(form: LenientForm<DataForm>, airports: State<'a, Vec<Airport>>)
    -> Result<Json<Vec<Route<'a>>>> {

    let form     = form.into_inner();
    let airports = airports.inner();

    if let (&Some(ref dep_icao), &Some(ref arr_icao)) = (&form.dep_icao, &form.arr_icao) {
        let route = Route::from_icao(dep_icao, arr_icao, form.mach, &airports)?;
        Ok(Json(vec![route]))
    } else {
        let start_time = PreciseTime::now();

        let departure = if let Some(ref dep_icao) = form.dep_icao {
            airport::find_by_icao(&dep_icao, airports)
                .ok_or("departure airport not found")?
        } else {
            airport::find(&form.to_enum(), airports)
                .ok_or("departure airport not found")?
        };

        let routes = route::find_all(departure, &form.to_enum(), form.mach, &airports)?;

        let end_time = PreciseTime::now();
        println!("filtering time: {} ms", start_time.to(end_time) * 1000);

        Ok(Json(routes))
    }
}

#[get("/countries")]
fn get_countries(countries: State<Vec<Country>>) -> Json<&Vec<Country>> {
    let countries = countries.inner();
    Json(countries)
}

fn init(rocket: Rocket) -> Result<Rocket> {
    let data = AirportData::create_and_update(Path::new("data"))?;
    let airport_data = data.parse_airports()?;

    let mut countries = data.parse_countries()?;
    countries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(rocket.manage(airport_data).manage(countries))
}

fn launch_rocket(rocket: Rocket) {
    rocket.mount("/", routes![index, files, get_countries, filter])
          .attach(Template::fairing())
          .launch();
}

fn main() {
    let rocket = rocket::ignite();

    match init(rocket) {
        Ok(rocket) => launch_rocket(rocket),
        Err(err) => {
            eprintln!("error: initialization failed\ninfo: {:?}", err);
        },
    }
}
