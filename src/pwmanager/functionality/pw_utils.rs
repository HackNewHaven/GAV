use rand::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

use gavlib::utils::rand_utils::random_string_of_len;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

   // Load the MongoDB connection string from an environment variable:

   let client_uri =

      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:

   // An extra line of code to work around a DNS issue on Windows:

   let options =

      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())

         .await?;

   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:

   println!("Databases:");

   for name in client.list_database_names(None, None).await? {

      println!("- {}", name);

   }

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
let passwords = client.databvase("user_passwords").collection("passwords");
}
