#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

mod route;
mod airport;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::Rocket;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use airport::data::DataFiles;

error_chain! {
    links {
        DataFiles(airport::data::Error, airport::data::ErrorKind);
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<i32, i32>::new())
}

fn init(rocket: Rocket) -> Result<Rocket> {
    let data_files = DataFiles::new(Path::new("data"))?;
    data_files.ensure_updated_data()?;

    let airport_data = data_files.parse()?;

    Ok(rocket.manage(airport_data))
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
