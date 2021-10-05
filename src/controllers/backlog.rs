use std::sync::Mutex;
use std::vec;
use crate::models::BacklogV2;
use crate::models::{
    BacklogInsert, 
    BacklogUpdate, 
    //AktivitasInsert,
    AktivitasInsertV2,
    ProjectInsert,
    CatatanInsert,
    MemberInsert
};
use actix_web::http::Error;
use actix_web::{HttpResponse, web};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use crate::{ConnectionPools, DbPool, JSONResult, LoginData, repositories::backlog};
use serde::{Deserialize, Serialize};
use crate::repositories::*;

#[derive(Deserialize)]
pub struct BacklogParams{
    nip_lama: Option<String>,
    nama_kerjaan: String,
    skor_urgensi: i32,
    skor_prediktabilitas: i32,
    skp_id: i32,
    sasaran_id: i32,
    urjab_id: i32,
    tgl_mulai: NaiveDate,
    tgl_selesai: Option<NaiveDate>,
    status: Option<String>,
}

#[derive(Deserialize)]
pub struct BacklogParamsV2{
    id_backlog: i32,
    nama_kerjaan: String,
    skor_urgensi: i32,
    skor_prediktabilitas: i32,
    skp_id: i32,
    sasaran_id: i32,
    urjab_id: i32,
    tgl_mulai: NaiveDate,
    tgl_selesai: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct AktivitasParams{
    niplama: Option<String>,
    nama_aktivitas: Option<String>,
    key_sort_unit: Option<String>,
    tanggal_aktivitas: NaiveDate,
}

#[derive(Deserialize)]
pub struct BacklogParamsId{
    id: i32,
}

#[derive(Serialize)]
struct BacklogResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct UserorgResponse<T> {
    success: bool,
    result: T,
}

#[derive(Deserialize)]
pub struct UserorgParamsId{
    id: i32,
}

#[derive(Serialize)]
struct AktivitasResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct SasaranResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct KinerjaResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct UrjabResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct BacklogResult {
    niplama: String,
    backlog: Vec<BacklogV2>
}

#[derive(Deserialize)]
pub struct IndikatorParam {
    id_sasaran: Option<i32>,
}

#[derive(Deserialize)]
pub struct NiplamaParam {
    niplama_bawahan: Option<String>,
}

#[derive(Deserialize)]
pub struct ProjectParams{
    judul: String,
    tgl_mulai: NaiveDate,
    tgl_selesai: Option<NaiveDate>,
    deadline: Option<NaiveDate>,
}

#[derive(Serialize)]
struct ProjectResponse<T> {
    success: bool,
    result: T,
}

#[derive(Deserialize)]
pub struct CatatanParams{
    id_project: i32,
    catatan: Option<String>,
    link_lampiran: Option<String>,
}

#[derive(Serialize)]
struct CatatanResponse<T> {
    success: bool,
    result: T,
}

#[derive(Deserialize)]
pub struct MemberParams{
    niplama: String,
}

#[derive(Serialize)]
struct MemberResponse<T> {
    success: bool,
    result: T,
}

#[derive(Serialize)]
pub struct ProjectId {
    pub project_id: String,
}

#[derive(Serialize)]
struct ProjectResponseV2 {
    pub id: i32,
    pub task_list: Vec<Backlog>,
    pub member_list: Vec<Member>,
    pub judul: String,
    pub tgl_mulai: NaiveDate,
    pub tgl_selesai: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub catatan_list: Vec<Catatan>,
}

#[derive(Serialize)]
pub struct Backlog{
    id: i32,
    nip_lama: Option<String>,
    nama: Option<String>,
    nip: Option<String>,
    nama_kerjaan: Option<String>,
    skor_urgensi: Option<i32>,
    urjab_id: Option<i32>,
    tgl_mulai: Option<NaiveDate>,
    tgl_selesai: Option<NaiveDate>,
    status: Option<i32>,
    total_skor: Option<i32>,
    skp_id: Option<i32>,
    sasaran_id: Option<i32>,
    kode_unit: Option<String>,
    skor_prediktabilitas: Option<i32>,
    id_project: i32,
}

#[derive(Serialize)]
pub struct Member{
    id_project: i32,
    niplama: String,
    nip: String,
    nama: String,
    unitkerja: String,
    peran: String,
}

#[derive(Serialize)]
pub struct Catatan{
    id_project: i32,
    catatan: Option<String>,
    link_lampiran: Option<String>,
}

pub async fn get_backlog(
    pool: web::Data<ConnectionPools>,
    login_data: web::Data<Mutex<LoginData>>
) -> Result<HttpResponse, Error> {
    let niplama = login_data.clone().lock().unwrap().user.clone().unwrap().user_nip.unwrap();
    let pegawai = employee::get_pegawai_by_niplama(&pool.hrm_pool, niplama.clone()).await;
    let bawahan = backlog::get_bawahan_by_kode_jabatan(&pool.hrm_pool, pegawai.unwrap().s_kd_jabdetail.clone()).await;
    let mut result = vec![];
    println!("{:?}", bawahan);
    if !bawahan.is_empty(){
        for peg in bawahan {
            println!("{:?}", peg.pegawai_id);
            let backlog = backlog::get_all_backlog_by_niplama(&pool.hrm_pool, peg.pegawai_id.clone().unwrap_or("".to_string())).await;
            println!("{:?}", backlog);
            if !backlog.is_empty(){
                result.push(BacklogResult{ niplama: peg.pegawai_id.unwrap_or("".to_string()), backlog});
            }
        }
    }else{
        let backlog = backlog::get_all_backlog_by_niplama(&pool.hrm_pool, niplama.clone()).await;
            if !backlog.is_empty(){
                result.push(BacklogResult{ niplama, backlog});
            }
    }

    let respons = BacklogResponse{
        success: true,
        result    
    };
    Ok(HttpResponse::Ok().json(respons))
}

pub async fn get_userorg(
    pool: web::Data<ConnectionPools>,
    
) -> Result<HttpResponse, Error> {
    let result = backlog::get_all_userorg(&pool.hrm_pool).await;
    let respons = UserorgResponse{
        success: true,
        result    
    };
    Ok(HttpResponse::Ok().json(respons))
}

#[derive(Serialize)]
pub struct BawahanList {
    pub niplama: String,
    pub nama: String,
    pub nip: String,
}

pub async fn get_list_bawahan(
    pool: web::Data<ConnectionPools>,
    login_data: web::Data<Mutex<LoginData>>
) -> Result<HttpResponse, Error> {
    let niplama = login_data.clone().lock().unwrap().user.clone().unwrap().user_nip.unwrap();
    let pegawai = backlog::get_pegawai_by_niplama(&pool.hrm_pool.clone(), niplama.clone()).await;
    let list_bawahan = backlog::get_bawahan_by_kode_jabatan(&pool.hrm_pool.clone(), pegawai ).await;
    let mut data_bawahan = vec![];
    for bawahan in list_bawahan {
        let data_pegawai = employee::get_pegawai_by_niplama(&pool.hrm_pool.clone(), bawahan.pegawai_id.unwrap_or("".to_string())).await;
        if let Some(data_pegawai) = data_pegawai {
            data_bawahan.push(BawahanList{ niplama: data_pegawai.niplama, nama: data_pegawai.nama, nip: data_pegawai.nip.unwrap_or("".to_string()) });
        }
    }
    let respons = UserorgResponse{
        success: true,
        result: data_bawahan    
    };
    Ok(HttpResponse::Ok().json(respons))
    // }else{
    //     let respons = UserorgResponse{
    //         success: false,
    //         result: "Data pegawai tidak ditemukan!"    
    //     };
    //     Ok(HttpResponse::Ok().json(respons))
    // }
}

pub async fn save_backlog(
    pool: web::Data<ConnectionPools>,
    params: web::Json<BacklogParams>,
    login_data: web::Data<Mutex<LoginData>>
)-> Result<HttpResponse, Error> {
    let mut niplama = login_data.clone().lock().unwrap().user.clone().unwrap().user_nip.unwrap();
    let mut kode_unit = "".to_string();
    let nip = params.nip_lama.clone();
    if nip.is_some(){
        niplama = nip.unwrap();
    }
    let data_pegawai = employee::get_pegawai_by_niplama(&pool.hrm_pool.clone(), niplama.clone()).await;
    if data_pegawai.is_some(){
        kode_unit = data_pegawai.unwrap().key_sort_unit.unwrap_or("".to_string());
    }

    let request = BacklogInsert{
        nip_lama: Some(niplama),
        nama_kerjaan : params.nama_kerjaan.clone(),
        skor_urgensi : params.skor_urgensi,
        urjab_id : params.urjab_id,
        tgl_mulai: params.tgl_mulai,
        tgl_selesai: params.tgl_selesai,
        status: Some(0),
        total_skor: params.skor_prediktabilitas * params.skor_urgensi,
        skp_id: params.skp_id,
        kode_unit,
        sasaran_id: params.sasaran_id,
        skor_prediktabilitas : params.skor_prediktabilitas,
    };
    
    let result= backlog::post_backlog(
        &pool.hrm_pool.clone(),
        request,
        params.nip_lama.clone(),
    ).await;

    let response = BacklogResponse {
        success: true,
        result,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_backlog_update(
    pool: web::Data<ConnectionPools>,
    params: web::Json<BacklogParamsV2>,
) -> Result<HttpResponse, Error> {
        let request = BacklogUpdate {
            // nip_lama: params.nip_lama.clone(),
            nama_kerjaan : params.nama_kerjaan.clone(),
            skor_urgensi : params.skor_urgensi,
            urjab_id : params.urjab_id,
            tgl_mulai: params.tgl_mulai,
            tgl_selesai: params.tgl_selesai,
            // status: params.status.clone(),
            total_skor: params.skor_prediktabilitas * params.skor_urgensi,
            skp_id: params.skp_id,
            sasaran_id: params.sasaran_id,
            skor_prediktabilitas : params.skor_prediktabilitas,
        };

        let result = backlog::post_backlog_update_v2(
            &pool.hrm_pool.clone(),
            request,
            params.id_backlog,
        )
        .await;

        let response = BacklogResponse {
            success: true,
            result,
        };
        Ok(HttpResponse::Ok().json(response))
}

pub async fn get_backlog_delete(
    pool: web::Data<ConnectionPools>,
    params: web::Json<BacklogParamsId>,
) -> Result<HttpResponse, Error> {
        let result = backlog::post_backlog_delete_v2(
            &pool.hrm_pool.clone(),
            params.id,
        )
        .await;

        let response = BacklogResponse {
            success: true,
            result,
        };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_backlog_done(
    pool: web::Data<ConnectionPools>,
    params: web::Json<BacklogParamsId>,
    //paramsId: web::Json<BacklogParamsId>,
)-> Result<HttpResponse, Error> {
    let backlog = backlog::get_backlog_by_id(&pool.hrm_pool.clone(), params.id).await;
    if let Some(backlog) = backlog {
        let request = AktivitasInsertV2{
            niplama: backlog.nip_lama.clone(),
            nama_aktivitas : backlog.nama_kerjaan.clone(),
            key_sort_unit : backlog.kode_unit.clone(),
            tanggal_aktivitas : Utc::now().naive_local().date(),
            id_b : Some(params.id),
            id_sasaran: backlog.sasaran_id.unwrap_or(0),
            id_sub_sasaran: backlog.skp_id.unwrap_or(0),
        };
        
        let result= backlog::post_aktivitas(
            &pool.hrm_pool.clone(),
            request,
           // paramsId.id_b,
        ).await;
    
        let response = AktivitasResponse {
            success: true,
            result,
        };
        Ok(HttpResponse::Ok().json(response))
    }else{
        let response = AktivitasResponse {
            success: false,
            result: "Data backlog tidak ditemukan",
        };
        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn get_sasaran(
    pool: web::Data<ConnectionPools>,
) -> Result<HttpResponse, Error> {
    let result = backlog::get_all_sasaran(&pool.hrm_pool).await;
    let respons = SasaranResponse{
        success: true,
        result    
    };
    Ok(HttpResponse::Ok().json(respons))
}

pub async fn get_indikator(
    pool: web::Data<ConnectionPools>,
    param: web::Query<IndikatorParam>,
) -> Result<HttpResponse, Error> {
    let result = backlog::get_all_indikator(&pool.hrm_pool, param.id_sasaran).await;
    let respons = KinerjaResponse{
        success: true,
        result    
    };
    Ok(HttpResponse::Ok().json(respons))
}

pub async fn get_uraian_jabatan(
    pool: web::Data<ConnectionPools>,
    login_data: web::Data<Mutex<LoginData>>,
    niplama_param: web::Query<NiplamaParam>
) -> Result<HttpResponse, Error> {
    let mut niplama = login_data.clone().lock().unwrap().user.clone().unwrap().user_nip.unwrap();
    let niplama_bawahan = niplama_param.niplama_bawahan.clone();
    if niplama_bawahan.is_some(){
        niplama = niplama_bawahan.unwrap()
    }
    let pegawai = backlog::get_pegawai_by_niplama(&pool.hrm_pool, niplama.clone()).await;
    let result = backlog::get_uraian_jabatan_by_kode(&pool.hrm_pool, pegawai.clone()).await;
    let respons = UrjabResponse{
        success: true,
        result    
    };
    Ok(HttpResponse::Ok().json(respons))
}

pub async fn get_project(
    pool: web::Data<ConnectionPools>,
) -> Result<HttpResponse, Error> {
    let project_result = backlog::get_all_project(&pool.hrm_pool).await;
    let mut results = Vec::new();
    for pro in project_result.iter(){
        let backlog_project = backlog::get_backlog_by_project(&pool.hrm_pool, pro.clone().id).await;
        let catatan_project = backlog::get_catatan_by_project(&pool.hrm_pool, pro.clone().id).await;
        let member_project = backlog::get_member_by_project(&pool.hrm_pool, pro.clone().id).await;
        
        let mut member_data = Vec::new();
        for mem in member_project.iter(){
            let mut member = Member{
                id_project:  mem.clone().id_project,
                niplama:  mem.clone().niplama,
                nip:  mem.clone().nip,
                nama:  mem.clone().nama,
                unitkerja:  mem.clone().unitkerja,
                peran:  mem.clone().peran,
            };
            member_data.push(member);
        }

        let mut catatan_data = Vec::new();
        for cat in catatan_project.iter(){
            let mut catatan = Catatan{
                id_project:  cat.clone().id_project,
                catatan: cat.clone().catatan,
                link_lampiran: cat.clone().link_lampiran,
            };
            catatan_data.push(catatan);
        }

        let mut backlog_data = Vec::new();
        for bac in backlog_project.iter(){
            let mut backlog = Backlog{

                id: bac.clone().id,
                nip_lama: bac.clone().nip_lama,
                nama: bac.clone().nama,
                nip: bac.clone().nip,
                nama_kerjaan: bac.clone().nama_kerjaan,
                skor_urgensi: bac.clone().skor_urgensi,
                urjab_id: bac.clone().urjab_id,
                tgl_mulai: bac.clone().tgl_mulai,
                tgl_selesai: bac.clone().tgl_selesai,
                status: bac.clone().status,
                total_skor: bac.clone().total_skor,
                skp_id: bac.clone().skp_id,
                sasaran_id: bac.clone().sasaran_id,
                kode_unit: bac.clone().kode_unit,
                skor_prediktabilitas: bac.clone().skor_prediktabilitas,
                id_project: bac.clone().id_project,
            };
            backlog_data.push(backlog);
        }
        
        results.push(ProjectResponseV2{
            id: pro.clone().id,
            task_list: backlog_data,
            member_list: member_data,
            judul: pro.clone().judul,
            tgl_mulai: pro.clone().tgl_mulai,
            tgl_selesai: pro.clone().tgl_selesai,
            deadline: pro.clone().deadline,
            catatan_list: catatan_data,
        })
    } 
    
    let respons = ProjectResponse{
        success: true,
        result: results
    };
    Ok(HttpResponse::Ok().json(respons))
}

pub async fn save_project(
    pool: web::Data<ConnectionPools>,
    params_pro: web::Json<ProjectParams>,
    params_cat: web::Json<CatatanParams>,
    params_mem: web::Json<MemberParams>,
)-> Result<HttpResponse, Error> {

    let request_pro = ProjectInsert{
        judul : params_pro.judul.clone(),
        tgl_mulai: params_pro.tgl_mulai,
        tgl_selesai: params_pro.tgl_selesai,
        deadline: params_pro.deadline,
    };

    let request_cat = CatatanInsert{
        id_project : params_cat.id_project,
        catatan: params_cat.catatan.clone(),
        link_lampiran: params_cat.link_lampiran.clone(),
    };

    let niplama = params_mem.niplama.clone();
    let data_pegawai = employee::get_pegawai_by_niplama(&pool.hrm_pool.clone(), niplama.clone()).await;
    let request_mem = MemberInsert{
        id_project : 1,
        niplama: niplama.clone(),
        nip: data_pegawai.clone().unwrap().nip.unwrap_or("".to_string()), 
        nama: data_pegawai.clone().unwrap().s_nama_lengkap.unwrap_or("".to_string()),
        unitkerja: data_pegawai.clone().unwrap().namaunit_lengkap.unwrap_or("".to_string()),
        peran: data_pegawai.clone().unwrap().namaunit_lengkap.unwrap_or("".to_string()),
    };
    
    let result= backlog::post_project(
        &pool.hrm_pool.clone(),
        request_pro,
    ).await;

    let result_cat= backlog::post_catatan(
        &pool.hrm_pool.clone(),
        request_cat,
    ).await;

    let result_mem= backlog::post_member(
        &pool.hrm_pool.clone(),
        request_mem,
    ).await;


    let response = ProjectResponse {
        success: true,
        result,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn save_catatan(
    pool: web::Data<ConnectionPools>,
    params: web::Json<CatatanParams>,
)-> Result<HttpResponse, Error> {

    let request = CatatanInsert{
        id_project : params.id_project,
        catatan: params.catatan.clone(),
        link_lampiran: params.link_lampiran.clone(),
    };
    
    let result= backlog::post_catatan(
        &pool.hrm_pool.clone(),
        request,
    ).await;

    let response = CatatanResponse {
        success: true,
        result,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn save_member(
    pool: web::Data<ConnectionPools>,
    params: web::Json<MemberParams>,
)-> Result<HttpResponse, Error> {
    let niplama = params.niplama.clone();
    let data_pegawai = employee::get_pegawai_by_niplama(&pool.hrm_pool.clone(), niplama.clone()).await;
    
    let request = MemberInsert{
        id_project : 1,
        niplama: niplama.clone(),
        nip: data_pegawai.clone().unwrap().nip.unwrap_or("".to_string()), 
        nama: data_pegawai.clone().unwrap().s_nama_lengkap.unwrap_or("".to_string()),
        unitkerja: data_pegawai.clone().unwrap().namaunit_lengkap.unwrap_or("".to_string()),
        peran: data_pegawai.clone().unwrap().namaunit_lengkap.unwrap_or("".to_string()),
    };
    
    let result= backlog::post_member(
        &pool.hrm_pool.clone(),
        request,
    ).await;

    let response = MemberResponse {
        success: true,
        result,
    };
    Ok(HttpResponse::Ok().json(response))
}

// pub async fn save_data_kesehatan_v2(
//     pool: web::Data<ConnectionPools>,
//     params: web::Json<KesehatanParamsV2>,
// ) -> Result<HttpResponse, Error> {
//     let niplama = &params.niplama;
//     let status = &params.status;
//     let gejala = &params.gejala;
//     let keterangan = &params.keterangan.clone().unwrap_or("".to_string());
//     let keterangan_komorbid = params.keterangan_komorbid.clone();
//     let is_komorbid = params.is_komorbid.clone();

//     kesehatan::save_covid_data_v2(
//         &pool.hrm_pool.clone(),
//         niplama.clone(),
//         status.clone(),
//         gejala.clone(),
//         keterangan.clone(),
//         keterangan_komorbid,
//         is_komorbid
//     )
//     .await;

//     let result =
//         kesehatan::get_latest_covid_by_niplama(&pool.hrm_pool.clone(), niplama.clone()).await;

//     if let Some(result) = result {
//         let response = JSONResult {
//             success: true,
//             result,
//         };
//         Ok(HttpResponse::Ok().json(response))
//     } else {
//         let response = JSONResult {
//             success: false,
//             result: "",
//         };
//         Ok(HttpResponse::NotFound().json(response))
//     }
// }