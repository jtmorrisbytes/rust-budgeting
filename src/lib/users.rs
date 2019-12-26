/*
    crate::lib::users.rs
    
    The set of actions that a user can perform are defined here

    The rules regarding the user actions will be in comments
    above the function describing the user action
*/





use diesel::PgConnection;
use crate::models::User;
// use crate::schema::users::dsl::*;


/*
    pub fn create_user(...)

    This function creates a new user in the database.
    In order for the user creation to be successful,
    the following requirements must be met:

    1. the user must supply a unique key (username) as a
       friendly way to distinguish one user from another
    2. The user must give at least their first name, with their last
       name being optional. This helps give the software a slightly
       more personal feel than calling the user (user 0000000)
    3. The user's username must not already exist in the database.
       if the user attempts to register again, instead direct them
       to login.
    4. Make sure to check that if for some reason a user id is already
       specified in the database even though the username is unique,
       that an error is thrown to alert the developer of a potental overwrite
       of data
    Future requirements:
    5. all personal data not dependant on looking up the user and is not relavent to the application
       SHOULD NOT be collected at ANY COST to prevent liability isses when and where possible
    6. Because data may be sent over the internet,
         THIS SOFTWARE IS NOT INTENDED FOR USE BY CHILDREN UNDER 13 * FOR ANY REASON * in order to
       protect from liability concerns. if a user is found to be under 13, they will have THREE DAYS
       from the inital date of contact to prove their age, otherwise their account will be deleted.

*/






pub fn exists_username<'a>(conn: &PgConnection, username: &'a str) -> bool {



    
}








pub fn get_all(connection: &PgConnection) -> (){
    // this function does nothing
    ()
}

// pub fn get_all(connection: &PgConnection) -> Result<Vec<User>,&'static str>{
//     users.load::<User>(connection)
// }
#[cfg(test)]
mod tests {
    use crate::lib::establish_connection;
    use crate::diesel::prelude::*;
    use crate::schema::users::dsl::*;
    use crate::models::User;
    #[test]
    fn test_create_user() {
        
        // use diesel::query_dsl::filter_dsl::FilterDsl;
        // use diesel::query_dsl::load_dsl::LoadQuery;
        let conn = establish_connection();
        /*
            get the list of users in the database to determine
             if this test should pass or fail.
            There should at least be one user with a userid of zero
            as the default user to test on.


            for more details see crate::models::User

        */

        let test_id = 1;
        let test_username = "testuser";
        let test_firstname = "test";
        let test_lastname = "runner";



        let baseline_results = users.filter(id.eq(test_id)).load::<User>(&conn).expect("Error loading users");
            
        if baseline_results.len() == 0 {
            //expect the create user method to succeed
        }
        else {
            for result in baseline_results {
            print!("{}",result);
            
            }

        }
        






        /*
         calling the function like this should produce a newly qualified user
         if the user id does not exist and the username is unique.
         the database should be set up to already have this user in the database,
         so it should fail. if it doesnt, please check that you have a userid 0 in the database
         and that the username is set to the same as below.


        */
        super::create_user(&conn,
            &test_id,
            &test_username,
            &test_firstname,
            &None);

    }
}