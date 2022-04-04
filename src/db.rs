use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_users() -> Vec<User> {
    let connection = establish_connection();
    users
        .filter(published.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts")
}
