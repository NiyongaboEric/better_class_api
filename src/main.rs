extern crate diesel;
extern crate dotenv;
extern crate serde_derive;

// use db::*;
use actix_web::{ web, App, HttpServer };
use diesel::prelude::*;
use diesel::r2d2::{ self, ConnectionManager };

// mod db;
mod users;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::post().to(users::handlers::add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
