/*
    User: the default user type.

          id:  A number used purely as an index for the database,
               ideally this will be managed by the database itself
               unless a situation arises later in development
               where the index will need to be manually specified
    username:  A user friendly name ( Required, Unique)
               that distinguishes one user from another
               using an [A-Za-Z0-9] character pattern.

               No special characters should be allowed here.
    firstname: The first name of the user (Required). $([A-Za-z]){3,25}^
               The software needs to know what to call the user.
               to make the experience feel more personal.

    lastname: The last name of the user (Optional). $([A-Za-z]){3,25}^
              the software requires at least one name
              but does not need more than that in
              the current version.


            


*/
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