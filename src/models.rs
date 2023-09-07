use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
  pub id: i32,
  pub hostname: String
}