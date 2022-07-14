use actix_web::{post, web, Scope, HttpResponse, Error};

use crate::{models::{DbPool, NewUser}, common};

#[post("/create")]
async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;

        common::add_user(&conn, &new_user.name)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

pub fn launch() -> Scope {
    web::scope("/user")
        .service(create_user)
}