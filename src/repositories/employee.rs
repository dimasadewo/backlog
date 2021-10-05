use crate::models::{
    Pegawai, 
    PegawaiTHL, 
    UnitKerja,
};
use crate::{actions, DbPool, repositories};
use actix_web::{web, Error, HttpResponse};
use serde::{Serialize, Deserialize};
use actix_web::error::ErrorInternalServerError;
use chrono::{NaiveDate, Utc, Datelike, NaiveDateTime, Duration};


pub async fn get_pegawai_by_niplama(
    pool: &DbPool,
    employee_id: String,
) -> Option<Pegawai> {
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let pegawai = web::block(move || actions::get_pegawai_by_user_nip(&conn, &employee_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");

    pegawai
}

pub async fn get_pegawai_by_kode_jabatan(
    pool: &DbPool,
    kode_jabatan: String,
) -> Option<Pegawai> {
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let pegawai = web::block(move || actions::get_pegawai_by_kode_jabatan(&conn, &kode_jabatan))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");

    pegawai
}

