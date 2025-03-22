use mysql::*;
use mysql::prelude::*;
use gavlib::utils::rand_utils::random_string_of_len;

struct user_db{
    username: String,
    password: String
}

fn connect_to_db() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    println!("Yay!");

    Ok(())
}    

// Generates a random password
pub fn generate_secure() -> String {
    // hardcoded length for now
    let len: usize = 12;
    random_string_of_len(len)
}

// Stores the password
pub fn store_password(){

}

// Retrive password
pub fn retrieve_password(){
}
