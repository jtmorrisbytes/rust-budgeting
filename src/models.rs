
use std::{i32};
use std::fmt;



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
pub const DEFAULT_DISPLAY: &'static str = "Null";

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let lastname: &mut String = String.clone_from(&default_display);
        match &self.lastname {
            Some(the_lastname) =>  write!(f, "{},{},{},{}", self.id,self.username,self.firstname,the_lastname),
            None => write!(f, "{},{},{},{}", self.id,self.username,self.firstname,"Null")
        }
        

    }
}


// pub struct NewUser {
//     pub id: i32,
//     pub username: String,
//     pub firstname: String,
//     pub lastname: Option<String>,

// }