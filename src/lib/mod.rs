



// pub mod models;
// pub mod schema;
use diesel::prelude::*;
/*  import requirements to use environment
    variables
*/
use dotenv::dotenv;
use std::env;


// make the user library functions usable
pub mod users;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}