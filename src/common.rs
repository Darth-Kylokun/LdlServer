use diesel::{Queryable, Insertable, sqlite::SqliteConnection};
use crate::schema::users;

#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub uuid: String,
    pub name: String
}

pub fn add_user<'r>(conn: &SqliteConnection) {
    
}