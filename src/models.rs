use diesel::prelude::*;
// use chrono::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
  pub id: i32,
  pub hostname: String,
  pub scraped: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::trips)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trips {
  pub id: i32,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::bookmarks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Bookmarks {
  pub id: i32,
  pub link: String,
  pub trip_id: i32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}