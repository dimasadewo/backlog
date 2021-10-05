use crate::models::{Pegawai, UnitKerja};
use crate::{actions, DbPool};
use actix_web::{web, Error, HttpResponse};

pub async fn get_unit_by_unit_code(
    pool: &DbPool,
    unit_code: String,
) -> Option<UnitKerja> {
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let response = web::block(move || actions::get_unitkerja_by_kode_unit(&conn, &unit_code))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Data not found!");
    response
}
