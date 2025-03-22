use mysql::*;
use mysql::prelude::*;
use gavlib::utils::rand_utils::random_string_of_len;

struct user_db{
    username: String,
    password: String,
    website: String
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
    //Sample insert w/o implementation on UI as we will do that later.
    
    let added_accounts = vec [
        user_db{username: "test1".to_string(), password: "test1".to_string(), website: "test1.com".to_string()}
    ];

    conn.exec_batch(
        r"INSERT INTO user_db (username, password, website)
          VALUES (:username, :password, :website)",
          added_accounts.iter().map(|p| params! {
            "username" => p.username,
            "password" => p.password,
            "website" => &p.website,
        })
    )?;
}

// Retrive password
pub fn retrieve_password(){
    //Sample select w/o implementation on UI as we will do that later.
    let selected_accounts = conn.query_map(
        "SELECT username, password, website FROM user_db",
        |(username, password, website)| {
            user_db {
                username,
                password,
                website
            }
        },
    )?;
    assert_eq!(added_accounts, selected_accounts);
}
