#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;


use std::env;   //this exposes the environment variable to our workspace
use dotenv::dotenv;

mod models;
mod schema;

fn main(){
    println!("Hello World");
}













    

