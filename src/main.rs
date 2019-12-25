
extern crate rust_budgeting;


// import requirements from diesel ORM

use self::diesel::prelude::*;
use diesel::pg::PgConnection;

/*
    import requirements from own package
*/
pub mod lib;
use self::lib;

use self::models::*;






fn main() {

    let connection = establish_connection();
    let results = 
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}



