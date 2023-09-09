use diesel_demo::*;
use std::io::stdin;

fn main() {
  let connection = &mut establish_connection();

  let mut host = String::new();

  println!("What website should be scraped");
  stdin().read_line(&mut host).unwrap();
  // host = host.trim_end();

  let website = create_website(connection, &host);
  println!("add {} with id {}", website.hostname, website.id);
}