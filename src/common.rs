use diesel::prelude::*;
use uuid::Uuid;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

use crate::models::User;

pub fn add_user<'r>(conn: &SqliteConnection, name: &'r str) -> Result<User, DbError> {
    use crate::schema::users;

    let new_user = User {
        uuid: Uuid::new_v4().to_string().to_owned(),
        name: name.to_string()
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}