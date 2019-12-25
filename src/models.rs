use diesel::helper_types::Nullable;
use std::{i32};





#[derive(Queryable)]
pub struct User {
    // current version uses SERIAL Type from postgres
    // the maximum value of SERIAL corrosponds with 
    // STD::i64::MAX
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: Option<String>
}