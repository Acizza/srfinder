#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate time;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

#[macro_use] mod util;
mod airport;
mod filter_form;
mod server;

use std::thread;

fn main() {
    let args = clap_app!(srfinder =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (@arg open: -o --open "Opens the web interface in the default web browser")
    ).get_matches();

    match server::init() {
        Ok(instance) => {
            if args.is_present("open") {
                let url = {
                    let config = instance.config();
                    format!("http://{}:{}", config.address, config.port)
                };

                let worker = thread::spawn(|| server::launch(instance));

                match util::url::open(&url) {
                    Ok(_)    => (),
                    Err(err) => eprintln!("error opening url: {:?}", err),
                }

                worker.join().unwrap();
            } else {
                server::launch(instance)
            }
        },
        Err(err) => {
            eprintln!("error initializing server: {:?}", err);
        }
    }
}
