#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

mod route;
mod airport;

use std::path::{Path, PathBuf};
use rocket::{Rocket, State};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use airport::data::{DataFiles, Country};

error_chain! {
    links {
        DataFiles(airport::data::Error, airport::data::ErrorKind);
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(Serialize)]
struct IndexTemplate<'a> {
    countries: &'a Vec<Country>,
}

#[get("/")]
fn index(countries: State<Vec<Country>>) -> Template {
    let context = IndexTemplate {
        countries: &countries.inner()
    };

    Template::render("index", &context)
}

fn init(rocket: Rocket) -> Result<Rocket> {
    let data_files = DataFiles::new(Path::new("data"))?;
    data_files.ensure_updated_data()?;

    let airport_data  = data_files.parse()?;
    let mut countries = data_files.parse_countries()?;

    countries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(rocket.manage(airport_data).manage(countries))
}

fn launch_rocket(rocket: Rocket) {
    rocket.mount("/", routes![index, route::filter_routes, files])
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
