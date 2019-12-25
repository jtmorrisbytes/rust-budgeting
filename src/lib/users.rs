


// this will eventually return the collection of


use diesel::PgConnection;
use crate::models::User;
use crate::schema::users::dsl::*;
use crate::diesel::RunQueryDsl;


pub fn get_all(connection: &PgConnection) -> () {
    // this function does nothing
    ()
}

// pub fn get_all(connection: &PgConnection) -> Result<Vec<User>,&'static str>{
//     users.load::<User>(connection)
// }