use std::{u64};





#[derive(diesel::Queryable)]
pub struct User {
    // current version uses SERIAL Type from postgres
    // the maximum value of SERIAL corrosponds with 
    // STD::i64::MAX
    pub id: u64,
    pub username: String,
    pub firstname: String,
    pub lastname: String
}