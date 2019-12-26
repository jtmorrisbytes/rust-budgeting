// extern crate rust_budgeting;
#[macro_use]
extern crate diesel;
extern crate dotenv;




use self::diesel::prelude::*;


pub mod lib;
pub mod models;
pub mod schema;

use self::lib::establish_connection;
use self::models::User;

use self::lib::users;





fn main() {
    // use self::schema::users::dsl::*;
    let connection = establish_connection();
    let results = users::get_all(&connection);
    // if results.len() == 0 {
    //     println!("No users found")
    // }
    // else {
    //     for user in results {
    //         println!(" userID: {} | username: {}, First Name: {}, Last Name: {:?} ",
    //                    user.id,user.username, user.firstname, user.lastname);
    //         println!("----------\n");
    //     }
    // }
    // println!("Displaying {} posts", results.len());
  
    // connection.
}



