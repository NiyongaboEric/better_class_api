// https://doc.rust-lang.org/rust-by-example/mod/split.html

// mod db;
// use crate::db;
// #[path = "db.rs"]
// use db;

use actix_web::Responder;

// mod users;
// #[path = "./models.rs"]
// mod models;
// use crate::src::db::models;÷

// use super::models::{ NewUser, User };
// use super::db::models::{ NewUser, User };
// use crate::db::schema::users::dsl::*;
// use crate::diesel::QueryDsl;
// use crate::diesel::RunQueryDsl;
// use super::Pool;
// use actix_web::{ web, Error, HttpResponse };
// use diesel::dsl::{ delete, insert_into };
// use serde::{ Deserialize, Serialize };
// use std::vec::Vec;


// use diesel::prelude::*;
// use diesel::r2d2::{ self, ConnectionManager };


// pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
// pub type Pool = Pool<ConnectionManager<PgConnection>>;÷

pub async fn add_user() -> impl Responder {
    format!("Hello from add user mehn 123")
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct InputUser {
//     pub username: String,
//     pub email: String,
// }

// pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//     Ok(web::block(move || get_all_users(db))
//         .await
//         .map(|user| HttpResponse::Ok().json(user))
//         .map_err(|_| HttpResponse::InternalServerError()?)
// }

// pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
//     let conn = pool.get().unwrap();
//     let items = users.load::<User>(&conn)?;
//     Ok(items)
// }

