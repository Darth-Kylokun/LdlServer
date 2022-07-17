use diesel::{Queryable, Insertable, r2d2::{self, ConnectionManager}, SqliteConnection};
use serde::{Serialize, Deserialize};
use crate::schema::users;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub uuid: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeColor {
    pub id: usize,
    pub color: u32
}