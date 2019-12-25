
use std::{i32};




use crate::schema::users;
#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="users"]
pub struct User {
    // current version uses SERIAL Type from postgres
    // the maximum value of SERIAL corrosponds with 
    // STD::i64::MAX
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: Option<String>
}




// pub struct NewUser {
//     pub id: i32,
//     pub username: String,
//     pub firstname: String,
//     pub lastname: Option<String>,

// }