use crate::{ConnectionPools, DbPool, controllers::backlog::BacklogParams};
use crate::models::{
    AktivitasInsert, 
    AktivitasInsertV2, 
    Backlog, 
    BacklogDelete, 
    BacklogInsert, 
    BacklogStatus, 
    BacklogUpdate, 
    BacklogV2, 
    DataKehadiranV4, 
    Indikator, 
    LoggingInsert, 
    Sasaran, 
    UraianJabatan, 
    Userorg, 
    UserorgV2,
    ProjectInsert,
    CatatanInsert,
    MemberInsert,
    ProjectBacklog,
    MemberProject,
    CatatanProject,
    BacklogProject

};
use crate::{actions};
use actix_web::{HttpResponse, web, get, post, Error};
use chrono::Utc;
use diesel::RunQueryDsl;
use serde::{Serialize, Deserialize};
use actix_web::error::ErrorInternalServerError;

use super::employee;

#[derive(Serialize)]
pub struct LogParamsV2{
    backlog_id: Option<i32>,
    keterangan: Option<String>,
    nip_lama: Option<String>,
}

pub async fn get_all_backlog(
    pool: &DbPool,
) -> Vec<BacklogV2> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let backlog = web::block(move || actions::get_all_backlog(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
        // tambah_logging(id, message)
    backlog
}

pub async fn get_all_backlog_by_niplama(
    pool: &DbPool,
    niplama: String,
) -> Vec<BacklogV2> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let backlog = web::block(move || actions::get_all_backlog_by_niplama(&conn, niplama.clone()))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
        // tambah_logging(id, message)
    backlog
}

pub async fn get_backlog_by_id(
    pool: &DbPool,
    id_backlog: i32,
) -> Option<BacklogV2> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let backlog = web::block(move || actions::get_backlog_by_id(&conn, id_backlog))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
        // tambah_logging(id, message)
    backlog
}

pub async fn get_pegawai_by_niplama(
    pool: &DbPool,
    niplama: String,
)-> Option<String> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let pegawai = web::block(move || actions::get_pegawai_by_niplama(&conn, niplama.clone()))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
        // tambah_logging(id, message)
    pegawai
}

pub async fn get_data_atasan_by_niplama(
    pool: &DbPool,
    niplama: Option<String>,
) -> Option<UserorgV2> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let atasan = web::block(move || actions::get_atasan_by_niplama(
        &conn, 
        niplama.clone().unwrap_or("".to_string()))
    )
    .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    atasan
}

pub async fn get_bawahan_by_kode_jabatan(
    pool: &DbPool,
    kode_jabatan: Option<String>,
) -> Vec<UserorgV2> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let atasan = web::block(move || actions::get_atasan_by_kode_jab(
        &conn, 
        kode_jabatan.clone().unwrap())
    )
    .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    atasan
}

pub async fn get_all_userorg(
    pool: &DbPool,
) -> Vec<Userorg> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let userorg = web::block(move || actions::get_all_userorg(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    userorg
}

pub async fn post_backlog(
    pool: &DbPool,
    request:BacklogInsert,
    nip_lama:Option<String>,
) -> BacklogInsert {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let backlog = web::block(move || actions::create_backlog(
        &conn,
        request,
        nip_lama.clone(),
     )).await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
        //create_logging_insert(&pool.hrm_pool.clone(), nip_lama.clone());
    backlog
}

pub async fn post_backlog_update_v2(
    pool: &DbPool,
    request: BacklogUpdate,
    id_backlog: i32,
) -> BacklogUpdate {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let pegawai = web::block(move || actions::post_backlog_update(
        &conn,
        request,
        id_backlog,
    ))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
    pegawai
}

pub async fn post_backlog_delete_v2(
    pool: &DbPool,
    id_b: i32,
) -> bool {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let params = BacklogDelete {
        deleted_at: Utc::now().naive_local(), 
    };

    web::block(move || actions::remove_backlog(
        &conn,
        params,
        id_b,
    ))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
    true
}

pub async fn post_aktivitas(
    pool: &DbPool,
    request: AktivitasInsertV2,
    // id_b: i32,
) -> AktivitasInsert {
    let conn1 = pool.get().expect("couldn't get db connection from pool");

    let data_atasan = get_data_atasan_by_niplama(&pool.clone(), request.niplama.clone()).await;
    let mut atasan1 = "".to_string();
    let mut atasan2 = "".to_string();
    let mut atasan3 = "".to_string();
    let mut niplama_atasan = None;
    let mut nama_atasan = None;
    let mut jabatan_atasan = None;
    let mut namaunit_atasan = None;
    if let Some(atasan) = data_atasan {
        atasan1 = atasan.kode_atasan.unwrap_or("".to_string());
        atasan2 = atasan.kode_atasan1.unwrap_or("".to_string());
        atasan3 = atasan.kode_atasan2.unwrap_or("".to_string());
        let atasan_langsung = employee::get_pegawai_by_kode_jabatan(&pool.clone(), atasan1.clone()).await;
        if let Some(atasan_langsung) = atasan_langsung {
            niplama_atasan = Some(atasan_langsung.niplama);
            nama_atasan = Some(atasan_langsung.nama);
            jabatan_atasan = atasan_langsung.jabatan;
            namaunit_atasan = Some(atasan_langsung.namaunit);
        }
    }

    let mut status_wfh = 0;
    let mut lat = None;
    let mut long = None;
    let mut sumber_lokasi = None;
    let kehadiran = get_data_presensi_hari_ini(&pool.clone(), request.niplama.clone().unwrap_or("".to_string())).await;
    if let Some(kehadiran) = kehadiran {
        status_wfh = kehadiran.mode_presensi.unwrap_or(0);
        lat = kehadiran.lat_datang;
        long = kehadiran.long_datang;
        if kehadiran.sumber_datang.unwrap_or(0) == 0 {
            sumber_lokasi = Some("Mobile".to_string());
        }else{
            sumber_lokasi = Some("Web".to_string());
        }
    }
    let requestV1 = AktivitasInsert{
        niplama: request.niplama.clone(),
        nama_aktivitas : request.nama_aktivitas.clone(),
        key_sort_unit : request.key_sort_unit.clone(),
        tanggal_aktivitas : request.tanggal_aktivitas,
        id_sasaran: request.id_sasaran,
        id_sub_sasaran: request.id_sub_sasaran,
        status_wfh,
        atasan1,
        atasan2,
        atasan3,
        niplama_atasan,
        nama_atasan,
        jabatan_atasan,
        namaunit_atasan,
        lat,
        long,
        sumber_lokasi,
    };

    let aktivtas = web::block(move ||  
        actions::create_aktivitas(&conn1, requestV1) 
        ).await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
        //post_backlog_status(&conn, id_b);

    let params = BacklogStatus { 
        status: 1,
        tgl_selesai: Utc::now().naive_local().date(),
    };

    let conn2 = pool.get().expect("couldn't get db connection from pool");
    let status = web::block(move || 
        actions::update_backlog_status(&conn2, params, request.id_b.unwrap_or(0),
    ))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
    aktivtas
}

pub async fn get_data_presensi_hari_ini(pool: &DbPool, niplama: String) -> Option<DataKehadiranV4> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let offset = get_timezone_offset_by_employee_id(niplama.clone(), pool.clone()).await;
    let tanggal = Utc::now().naive_local().date();
    let result = web::block(move || actions::get_attendance_record_v4(&conn, offset, niplama.clone(), tanggal)).await
.map_err(|e| {
    eprintln!("{}", e);
    HttpResponse::InternalServerError().finish()
}).expect("Koneksi gagal!");
    result
}

pub async fn get_timezone_offset_by_employee_id(employee_id: String, pool: DbPool) -> i64 {
    let zona_waktu = get_timezone_for_pegawai(employee_id.clone(), pool.clone()).await;
    let offset: i64;
    match zona_waktu.as_str() {
        "WIB" => offset = 7,
        "WITA" => offset = 8,
        "WIT" => offset = 9,
        _ => offset = 7,
    }
    offset
}

pub async fn get_timezone_for_pegawai(employee_id: String, pool: DbPool) -> String {
    let niplama = employee_id.clone();
    let conn = pool.get().expect("couldn't get DB connection from pool");
    let kode_unit = web::block(move || actions::get_pegawai_by_user_nip(&conn, niplama.as_str()))
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Data not found");
    if let Some(kode_unit) = kode_unit {
        let conn = pool.get().expect("couldn't get DB connection from pool");
        let zona_waktu = web::block(move || {
            actions::get_unitkerja_by_kode_unit(&conn, kode_unit.key_sort_unit.unwrap().as_str())
        })
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Data not found");
        if let Some(zona_waktu) = zona_waktu {
            return zona_waktu.zona_waktu;
        } else {
            return "WIB".to_string();
        }
    } else {
        return "WIB".to_string();
    }
}

// pub async fn create_logging_insert(
//     pool: &DbPool,
//     niplama: String,
// )-> bool {
//     let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
//     let message = format!("User {} telah melakukan GET atas seluruh data backlog", niplama);

//     let request = LoggingInsert{
//         backlog_id: 20,
//         keterangan: message,
//         nip_lama: niplama,
//     };

//     let log = web::block(move || actions::create_logging_list(&conn, request))
//         .await
//         .map_err(|e| {
//             eprintln!("{}",e);
//             HttpResponse::InternalServerError().finish()
//         })
//         .expect("Connection failed!");
//     true
// }

pub async fn get_all_sasaran(
    pool: &DbPool,
) -> Vec<Sasaran> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let sasaran = web::block(move || actions::get_all_sasaran(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    sasaran
}

pub async fn get_all_indikator(
    pool: &DbPool,
    id_sasaran: Option<i32>,
) -> Vec<Indikator> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let id = id_sasaran.unwrap_or(0);
    let indikator = web::block(move || actions::get_all_indikator(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    indikator
}

pub async fn get_uraian_jabatan_by_kode(
    pool: &DbPool,
    kode_jabatan: Option<String>,
) -> Vec<UraianJabatan> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let urjab = web::block(move || actions::get_uraian_jabatan_by_kode(
        &conn,
        kode_jabatan.clone().unwrap()
    ))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    urjab
}

pub async fn get_all_project(
    pool: &DbPool,
) -> Vec<ProjectBacklog> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let project = web::block(move || actions::get_all_project(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    project
}

pub async fn post_project(
    pool: &DbPool,
    request:ProjectInsert,
) -> ProjectInsert {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let project = web::block(move || actions::create_project(
        &conn,
        request,
    )).await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
    project
}

pub async fn get_backlog_by_project(
    pool: &DbPool,
    id_project: i32,
) ->  Vec<BacklogProject> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let backlog = web::block(move || actions::get_backlog_by_project(
        &conn, 
        id_project
    )).await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    backlog
}

pub async fn get_catatan_by_project(
    pool: &DbPool,
    id_project: i32,
) ->  Vec<CatatanProject> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let catatan = web::block(move || actions::get_catatan_by_project(
        &conn, 
        id_project
    )).await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    catatan
}

pub async fn post_catatan(
    pool: &DbPool,
    request:CatatanInsert,
   // nip_lama:Option<String>,
) -> CatatanInsert {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let catatan = web::block(move || actions::create_catatan(
        &conn,
        request,
       // nip_lama.clone(),
    )).await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
        //create_logging_insert(&pool.hrm_pool.clone(), nip_lama.clone());
    catatan
}

pub async fn get_member_by_project(
    pool: &DbPool,
    id_project: i32,
) ->  Vec<MemberProject> {
    let conn = pool.get().expect("Gagal mendapatkan koneksi database!");
    let member = web::block(move || actions::get_member_by_project(
        &conn, 
        id_project
    ))
    .await
        .map_err(|e| {
            eprintln!("{}",e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Connection failed!");
    member
}

pub async fn post_member(
    pool: &DbPool,
    request:MemberInsert,
   // nip_lama:Option<String>,
) -> MemberInsert {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let member = web::block(move || actions::create_member(
        &conn,
        request,
       // nip_lama.clone(),
    )).await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }).expect("Koneksi Gagal");
        //create_logging_insert(&pool.hrm_pool.clone(), nip_lama.clone());
    member
}