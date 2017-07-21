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
use rocket::response::NamedFile;
use rocket_contrib::Template;
use airport::data::DataFiles;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<i32, i32>::new())
}

fn main() {
    let data_files = DataFiles::new(Path::new("data")).unwrap();
    data_files.ensure_updated_data().unwrap();

    let result = data_files.parse().unwrap();

    println!("{:?}", result.iter().find(|a| &a.icao == "KSMF"));
    println!("num: {}", result.len());

    rocket::ignite()
        .mount("/", routes![index, route::parse_filters, files])
        .attach(Template::fairing())
        .launch();
}
