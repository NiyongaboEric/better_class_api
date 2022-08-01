#[macro_use]
use diesel;

use crate::schema::*;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Queryable)]
struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
