#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_derive;

mod db;
mod handlers;

use db::*;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder };

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("Rust_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/users", web::post().to(handlers::add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await 
}
