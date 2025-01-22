use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::pg::PgConnection;

pub fn create_pool() -> PgConnection {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // match PgConnection::establish(&database_url) {
    //     Ok(_) => println!("Successfully connected to the database! 🎉"),
    //     Err(err) => eprintln!("Failed to connect to the database: {}", err),
    // }

    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



