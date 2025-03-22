use rand::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

// Generates a random password
pub fn generate_secure()-> String{
    let mut rng = rand::rng();
    let password: String 

}

// Stores the password
pub fn store_password(){

}

// Retrive password
pub fn retrieve_password(){

}