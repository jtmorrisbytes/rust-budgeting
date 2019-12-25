extern crate rust_budgeting;
extern crate diesel;
use self::diesel::prelude::*;
use self::rust_budgeting::*;



use self::models::User;
// import requirements from diesel ORM


/*
    import requirements from own package
*/






fn main() {
    use rust_budgeting::schema::users::dsl::*;
    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading Users");
    if results.len() == 0 {
        println!("No users found")
    }
    else {
        for user in results {
            println!(" userID: {} | username: {}, First Name: {}, Last Name: {:?} ",
                       user.id,user.username, user.firstname, user.lastname);
            println!("----------\n");
        }
    }
    // println!("Displaying {} posts", results.len());
  
    // connection.
}



