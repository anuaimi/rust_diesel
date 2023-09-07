use diesel_demo::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {

  use self::schema::websites::dsl::*;

  let connection = &mut establish_connection();
  let results = websites
        .limit(5)
        .select(Website::as_select())
        .load(connection)
        .expect("Error loading websites");

  println!("Displaying {} websites", results.len());
  for website in results {
    println!("{}", website.hostname);
  }
}