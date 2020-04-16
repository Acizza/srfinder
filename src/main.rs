mod airport_data;
mod path;
mod server_route;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use server_route::search_routes::search_routes;
use std::env;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    if let Err(err) = airport_data::ensure_updated() {
        panic!("airport data update failed: {}", err);
    }

    println!("loading airport data..");

    let airports = match airport_data::Airport::load_all() {
        Ok(airports) => web::Data::new(airports),
        Err(err) => panic!("error loading airport data: {}", err),
    };

    println!("finished loading airport data..");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(airports.clone())
            .data(web::JsonConfig::default().limit(2048))
            .service(search_routes)
            .service(fs::Files::new("/", "./frontend/dist/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
