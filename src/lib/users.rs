


// this will eventually return the collection of


use diesel::PgConnection;
use crate::models::User;
// use crate::schema::users::dsl::*;
use crate::diesel::RunQueryDsl;


pub fn create_user<'a>(conn : &PgConnection,
                       id: &'a i32,
                       username: &'a str,
                       first_name: &'a str,
                       last_name: &'a Option<String>) -> () {

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
            ass
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
            &0,
            &"jthec",
            &"jordan",
            &None);

    }
}