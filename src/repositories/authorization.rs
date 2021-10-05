use crate::{actions, DbPool};
use actix_web::{web, Error, HttpResponse};

pub async fn authorize_token(token: String, pool: &DbPool) -> Result<bool, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get any mysql connection from pool!");
    let user = web::block(move || actions::get_user_by_token(&conn, &token))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if user.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

