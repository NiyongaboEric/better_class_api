#[macro_use];
extern crate diesel;
extern crate dotenv;
extern crate serde_derive;

mod db;
use db::*;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder };

fn index() -> impl Responder {
    let users = get_users();
    HttpResponse::Ok().json(users)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
