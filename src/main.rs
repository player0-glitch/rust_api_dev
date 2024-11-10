#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

//allows us to use environment variables
use std::env;

use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;

mod schema;
mod model;

pub fn establish_connection()->SqliteConnection{
    dotenv().ok();

    let db_url=env::var("DATABASE_URL").expect("DATABASE_URL needs to be set");
        SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Cannot Connect to database at {}",db_url))
}
fn main(){
    println!("Hello World");
}













    

