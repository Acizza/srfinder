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
mod server;

fn main() {
    match server::launch() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("fatal error: {:?}", err);
        }
    }
}
