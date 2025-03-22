use rand::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn connect_to_db() -> Result<(), Box<dyn error>> {
    let client_uri =
        env::car("DB_URI").expect("Please setup MongoDB_URI enviorment variable!");

        let options =
            ClientOptions::parse_with_resolver_config(client_uri, ResolverConfig::cloudflare()).await?;
            let client = Client::with::options(options)?;
        }

    ok(())
// Generates a random password
pub fn generate_secure()-> String {
    let mut rng = rand::rng();
    let password: String;

}

// Stores the password
pub fn store_password(){

}

// Retrive password
pub fn retrieve_password(){
let passwords = client.databvase("user_passwords").collection("passwords");
}
