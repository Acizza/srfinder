#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate time;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

mod filter;
mod util;

use filter::airport::Airport;
use filter::data::{Country, DataFiles};
use filter::DataForm;
use filter::route::Route;
use rocket_contrib::{Template, Json};
use rocket::{Rocket, State};
use rocket::response::NamedFile;
use rocket::request::LenientForm;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use time::PreciseTime;

error_chain! {
    links {
        DataFiles(filter::data::Error, filter::data::ErrorKind);
        Route(filter::route::Error, filter::route::ErrorKind);
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

    let start_time = PreciseTime::now();
    let routes     = Route::get_all_filter_matches(&form, &airports)?;
    let end_time   = PreciseTime::now();
    println!("filtering time: {} ms", start_time.to(end_time) * 1000);

    Ok(Json(routes))
}

#[get("/countries")]
fn get_countries(countries: State<Vec<Country>>) -> Json<&Vec<Country>> {
    let countries = countries.inner();
    Json(countries)
}

fn init(rocket: Rocket) -> Result<Rocket> {
    let data_files   = DataFiles::create_and_verify(Path::new("data"))?;
    let airport_data = data_files.parse_airports()?;

    let mut countries = data_files.parse_countries()?;
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
