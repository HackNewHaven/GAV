use rand::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

use gavlib::utils::rand_utils::random_string_of_len;

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
