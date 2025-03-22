use rand::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

use gavlib::utils::rand_utils::random_string_of_len;
#[tokio::main]
pub async fn connect_to_db() -> Result<Client, Box<dyn Error>> {
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let mut client_options = ClientOptions::parse_with_resolver_config(&db_url, ResolverConfig::cloudflare()).await?;
    client_options.app_name = Some("PasswordManager".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)

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
let passwords = client.databvase("user_passwords").collection("passwords");
}
