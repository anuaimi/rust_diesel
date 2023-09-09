use self::models::{NewWebsite, Website};
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect(".env file not found");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_website(conn: &mut PgConnection, hostname: &String) -> Website {
    use crate::schema::websites;

    let new_website = NewWebsite{hostname};
    diesel::insert_into(websites::table)
        .values(&new_website)
        .returning(Website::as_returning())
        .get_result(conn)
        .expect("Error saving new website")
}
