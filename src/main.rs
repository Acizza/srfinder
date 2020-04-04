mod search_routes;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use std::env;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(2048))
            .service(search_routes::search_routes)
            .service(fs::Files::new("/", "./frontend/dist/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
