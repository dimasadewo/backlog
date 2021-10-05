use diesel::dsl::sql;
use diesel::prelude::*;

use crate::{models};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use std::convert::TryInto;
use diesel::dsl::date;
use crate::models::{
    FingerprintUpdate, 
    FingerprintTHLUpdate, 
    FingerprintTHLCreate, 
    FingerprintCreate, 
    TransportBaru,
    BacklogInsert,
    Backlog,
    BacklogUpdate,
    BacklogDelete, 
    Userorg,
    UserorgV2,
    LoggingInsert,
    AktivitasInsert,
    AktivitasInsertV2,
    Sasaran,
    Indikator,
    BacklogStatus,
    UraianJabatan,
    ProjectInsert,
    CatatanInsert,
    MemberInsert,
    ProjectBacklog,
    MemberProject,
    CatatanProject,
    BacklogProject
};

use diesel::sql_query;
use diesel::sql_types::Text;

pub fn get_all_users(
    conn: &MysqlConnection,
) -> Result<Option<Vec<models::User>>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let result = users.load::<models::User>(conn).optional()?;

    Ok(result)
}

pub fn get_user_by_employee_id(
    conn: &MysqlConnection,
    employee_id: &str,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let result = users
        .filter(user_nip.eq(employee_id))
        .first::<models::User>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_user_by_token(
    conn: &MysqlConnection,
    token: &str,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let result = users
        .filter(api_token.eq(token))
        .first::<models::User>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_all_pegawais(
    conn: &MysqlConnection,
) -> Result<Option<Vec<models::Pegawai>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .filter(status.eq("Aktif"))
        .load::<models::Pegawai>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_all_pegawais_employee_id(
    conn: &MysqlConnection,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .select(niplama)
        .filter(status.eq("Aktif"))
        .load::<String>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_first_count_pegawais(
    conn: &MysqlConnection,
    count: u32,
) -> Result<Option<Vec<models::Pegawai>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .filter(status.eq("Aktif"))
        .limit(count.try_into().unwrap())
        .load::<models::Pegawai>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_pegawai_by_user_nip(
    conn: &MysqlConnection,
    user_nip: &str,
) -> Result<Option<models::Pegawai>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .filter(niplama.eq(user_nip))
        .first::<models::Pegawai>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_pegawai_thl_by_user_nip(
    conn: &MysqlConnection,
    user_nip: &str,
) -> Result<Option<models::PegawaiTHL>, diesel::result::Error> {
    use crate::schema::ren_peg_last_thl::dsl::*;

    let result = ren_peg_last_thl
        .filter(nip.eq(user_nip))
        .first::<models::PegawaiTHL>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_pegawai_by_kode_jabatan(
    conn: &MysqlConnection,
    kode_jabatan: &str,
) -> Result<Option<models::Pegawai>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .filter(s_kd_jabdetail.eq(kode_jabatan))
        .first::<models::Pegawai>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_pegawai_by_nipbaru(
    conn: &MysqlConnection,
    nipbaru_param: &str,
) -> Result<Option<models::Pegawai>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .filter(nip.eq(nipbaru_param))
        .first::<models::Pegawai>(conn)
        .optional()?;

    Ok(result)
}

// pub fn get_data_kode_atasan(
//     conn: &MysqlConnection,
//     user_nip: &str,
// ) -> Result<Option<models::KodeAtasan>, diesel::result::Error> {
//     use crate::schema::t_userorg::dsl::*;

//     let result = t_userorg
//         .filter(pegawai_id.eq(user_nip))
//         .first::<models::KodeAtasan>(conn)
//         .optional()?;

//     Ok(result)
// }

// pub fn get_subordinates_by_employee_position_id(
//     conn: &MysqlConnection,
//     employee_position_code: &str,
// ) -> Result<Option<Vec<models::KodeAtasan>>, diesel::result::Error> {
//     use crate::schema::t_userorg::dsl::*;

//     let result = t_userorg
//         .filter(kode_atasan.eq(employee_position_code))
//         .or_filter(kode_atasan1.eq(employee_position_code))
//         .or_filter(kode_atasan2.eq(employee_position_code))
//         .load::<models::KodeAtasan>(conn)
//         .optional()?;
//     Ok(result)
// }

// pub fn get_subordinates_by_employee_position_id_and_subs_id(
//     conn: &MysqlConnection,
//     employee_position_code: &str,
//     employee_active_id: Vec<String>,
// ) -> Result<Option<Vec<String>>, diesel::result::Error> {
//     use crate::schema::t_userorg::dsl::*;

//     let result = t_userorg
//         .select(pegawai_id)
//         .filter(kode_atasan
//             .eq(employee_position_code)
//             .and(pegawai_id.eq_any(employee_active_id.clone()))
//         )
//         .or_filter(kode_atasan1
//             .eq(employee_position_code)
//             .and(pegawai_id.eq_any(employee_active_id.clone()))
//         )
//         .or_filter(kode_atasan2
//             .eq(employee_position_code)
//             .and(pegawai_id.eq_any(employee_active_id.clone()))
//         )
//         .load::<String>(conn)
//         .optional()?;
//     Ok(result)
// }

pub fn get_permissions(
    conn: &MysqlConnection,
    fungsi_param: &str,
    modul_param: &str,
) -> Result<Option<Vec<models::Permission>>, diesel::result::Error> {
    use crate::schema::t_permission::dsl::*;

    let result = t_permission
        .filter(fungsi.eq(fungsi_param))
        .filter(modul.eq(modul_param))
        .load::<models::Permission>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_attendance_record_within_periods(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
    tanggal_awal: NaiveDate,
    tanggal_akhir: Option<NaiveDate>
) -> Result<Option<Vec<models::Fingerprint>>, diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    let wektu_awal = NaiveDate::from_ymd(tanggal_awal.year(), tanggal_awal.month(), tanggal_awal.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap().to_owned();
    let mut wektu_akhir = Utc::now().naive_utc();
    if tanggal_akhir.is_some(){
        wektu_akhir = NaiveDate::from_ymd(tanggal_akhir.unwrap().year(), tanggal_akhir.unwrap().month(), tanggal_akhir.unwrap().day())
            .and_hms(0, 0, 0)
            .checked_add_signed(Duration::hours(offset)).unwrap().to_owned();
    }

    println!("Wektu akhir: {}", wektu_akhir);

    let result = t_fingerprint_android
        .order(id.asc())
        .filter(niplama.eq(employee_id))
        .filter(created_at.ge(wektu_awal))
        .filter(created_at.le(wektu_akhir))
        .load::<models::Fingerprint>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_attendance_record_by_date(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
    tanggal: NaiveDate,
) -> Result<Option<models::Fingerprint>, diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    let wektu = NaiveDate::from_ymd(tanggal.year(), tanggal.month(), tanggal.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap().to_owned();
    let wektu_sesuk = wektu.clone().checked_add_signed(Duration::days(1))
        .unwrap().to_owned();

    let result = t_fingerprint_android
        .order(id.asc())
        .filter(niplama.eq(employee_id))
        .filter(created_at.ge(wektu))
        .filter(created_at.le(wektu_sesuk))
        .first::<models::Fingerprint>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_attendance_thl_record_by_date(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
    tanggal: NaiveDate,
) -> Result<Option<models::FingerprintTHL>, diesel::result::Error> {
    use crate::schema::t_fingerprint_thl::dsl::*;

    let wektu = NaiveDate::from_ymd(tanggal.year(), tanggal.month(), tanggal.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap().to_owned();
    let wektu_sesuk = wektu.clone().checked_add_signed(Duration::days(1))
        .unwrap().to_owned();

    let result = t_fingerprint_thl
        .order(id.asc())
        .filter(nip.eq(employee_id))
        .filter(created_at.ge(wektu))
        .filter(created_at.le(wektu_sesuk))
        .first::<models::FingerprintTHL>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_todays_attendance_record(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
) -> Result<Option<models::Fingerprint>, diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    let latest = t_fingerprint_android.order(id.desc()).filter(niplama.eq(employee_id)).first::<models::Fingerprint>(conn.clone())?;
    println!("Latest! : {:?}", latest);

    // let saat_ini = Local::now();
    // println!("Saat ini: {}", saat_ini);
    let now = Utc::now();
    let hari_ini = now.checked_add_signed(Duration::hours(offset)).unwrap().date().naive_utc();
    let wektu_ini = NaiveDate::from_ymd(hari_ini.year(), hari_ini.month(), hari_ini.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap().to_owned();
    println!("Hari ini: {}", hari_ini);
    println!("Wektu ini: {}", wektu_ini);
    println!("Nauw!: {}", now);

    let result = t_fingerprint_android
        .order(id.asc())
        .filter(niplama.eq(employee_id))
        .filter(created_at.ge(wektu_ini))
        .first::<models::Fingerprint>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_todays_attendance_thl_record(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
) -> Result<Option<models::FingerprintTHL>, diesel::result::Error> {
    use crate::schema::t_fingerprint_thl::dsl::*;

    let now = Utc::now();
    let hari_ini = now.checked_add_signed(Duration::hours(offset)).unwrap().date().naive_utc();
    let wektu_ini = NaiveDate::from_ymd(hari_ini.year(), hari_ini.month(), hari_ini.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap().to_owned();
    println!("Hari ini: {}", hari_ini);
    println!("Wektu ini: {}", wektu_ini);
    println!("Nauw!: {}", now);

    let result = t_fingerprint_thl
        .order(id.asc())
        .filter(nip.eq(employee_id))
        .filter(created_at.ge(wektu_ini))
        .first::<models::FingerprintTHL>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_national_holiday_including_facultative(
    conn: &MysqlConnection,
    date: NaiveDate,
    unit_id: i32,
) -> Result<Option<models::Holiday>, diesel::result::Error> {
    use crate::schema::t_cuti_bersama::dsl::*;
    let result = t_cuti_bersama
        .filter(tanggal.eq(date))
        .filter(ref_unitkerja_id.eq(1).or(ref_unitkerja_id.eq(unit_id)))
        .first::<models::Holiday>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_todays_attendance_record_minus_n(
    conn: &MysqlConnection,
    offset: i64,
    employee_id: &str,
    days: i64,
) -> Result<Option<models::Fingerprint>, diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    let now = Utc::now();
    let hari_ini = now.checked_add_signed(Duration::hours(offset)).unwrap().date().naive_utc();
    let wektu_ini = NaiveDate::from_ymd(hari_ini.year(), hari_ini.month(), hari_ini.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset)).unwrap()
        .checked_sub_signed(Duration::days(days)).unwrap()
        .to_owned();
    let wektu_sesuk = wektu_ini.clone().checked_add_signed(Duration::days(1))
        .unwrap().to_owned();
    println!("Hari ini: {}", hari_ini);
    println!("Wektu ini: {}", wektu_ini);

    let result = t_fingerprint_android
        .order(id.asc())
        .filter(niplama.eq(employee_id))
        // .filter(date(created_at).eq(hari_ini))
        .filter(created_at.ge(wektu_ini))
        .filter(created_at.le(wektu_sesuk))
        .first::<models::Fingerprint>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_leave_allowance_for_year_minus_n(
    conn: &MysqlConnection,
    employee_id: &str,
    minus: i32,
    current: bool,
) -> Result<Option<models::LeaveAllowance>, diesel::result::Error> {
    let mut leave_code = "101";
    if minus > 0 {
        leave_code = "102";
    }
    if current {
        current_leave(conn, minus, employee_id)
    } else {
        latest_leave(conn, minus, employee_id, leave_code)
    }
}

pub fn get_suspended_leave_request_for_current_year(
    conn: &MysqlConnection,
    employee_id: &str,
) -> Result<Option<models::LeaveAllowance>, diesel::result::Error> {
    use crate::schema::t_saldo_cuti::dsl::*;
    let today = Local::today();
    let result_year = today.year() - 1;
    let result = t_saldo_cuti
        .order(id.desc())
        .filter(niplama_pegawai.eq(employee_id))
        .filter(kode_cuti.eq("103"))
        .filter(tahun.eq(result_year))
        .first::<models::LeaveAllowance>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_subordinates_open_work_leave_request(
    conn: &MysqlConnection,
    employee_long_id: &str,
) -> Result<Option<Vec<models::WorkLeave>>, diesel::result::Error> {
    use crate::schema::t_cuti::dsl::*;
    let result = t_cuti
        .order(id.desc())
        .filter(
            nip_atasan
                .eq(employee_long_id)
                .and(kd_proses.eq(0))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            nip_atasan2
                .eq(employee_long_id)
                .and(kd_proses.eq(1))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            nip_atasan3
                .eq(employee_long_id)
                .and(kd_proses.eq(2))
                .and(deleted_at.is_null()),
        )
        .load::<models::WorkLeave>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_list_pengajuan_cuti_ds_bawahan(
    conn: &MysqlConnection,
    employee_long_id: &str,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::t_cuti::dsl::*;
    let result = t_cuti
        .order(id.desc())
        .filter(
            nip_atasan3
                .eq(employee_long_id)
                .and(kd_proses.eq(2))
                .and(deleted_at.is_null()),
        )
        .select(niplama_pegawai)
        .load::<String>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_list_pengajuan_cuti_by_kode_unit(
    conn: &MysqlConnection,
    kode_unit_param: &str,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::t_cuti::dsl::*;
    let result = t_cuti
        .order(id.desc())
        .filter(
            kode_unit
                .eq(kode_unit_param)
                .and(deleted_at.is_null()),
        )
        .select(niplama_pegawai)
        .load::<String>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_pengajuan_kgb_pegawai(
    conn: &MysqlConnection,
    kode_jabatan: &str,
    jabatan_param: Option<String>,
    kode_unit: Option<String>,
    niplama_pegawai: Option<&str>,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;
    let table = ren_peg_last
        .select(niplama)
        .into_boxed();
    let mut result;

    match kode_jabatan {
        "240107001500001100059" => result = table.filter(golruang.like("III/%"))
            .filter(namaunit.not_like("Perwakilan%"))
            .filter(namaunit.not_like("Pusat%")),
        "230107001500001100021" => result = table.filter(golruang.like("II/%"))
            .filter(namaunit.not_like("Perwakilan%"))
            .filter(namaunit.not_like("Pusat%")),
        "220107001500001100007" => result = table.filter(golruang.like("III/%")
            .or(jenis_jab.eq("E.III.a"))
        ).or_filter(golruang.like("IV/%")
            .and(jenis_jab.eq("E.III.a"))
        ).filter(namaunit.not_like("Perwakilan%"))
            .filter(namaunit.not_like("Pusat%")),
        "210107001500001100001" => result = table.filter(jenis_jab.eq("E.II.a")),
        "210107001500001100001" => result = table.filter(jenis_jab.eq("E.I.a")),
        _ => result = if jabatan_param.clone().unwrap().contains("Kepala Perwakilan") || jabatan_param.unwrap().contains("Kepala Pusat") {
            table.filter(key_sort_unit.eq(kode_unit)).filter(jenis_jab.ne("E.II.a")).filter(jenis_jab.ne("E.I.a")).filter(status.eq("Aktif"))
        }else{
            table.filter(niplama.eq(niplama_pegawai.unwrap()))
        }
    }

    let kgb_pegawai = result.load::<String>(conn).optional()?;

    Ok(kgb_pegawai)
}

pub fn get_pengajuan_kgb_by_kode_unit(
    conn: &MysqlConnection,
    kode_unit: &str,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;
    let result = ren_peg_last
        .select(niplama)
        .filter(key_sort_unit.eq(kode_unit)).filter(status.eq("Aktif")).load::<String>(conn).optional()?;

    Ok(result)
}

pub fn get_pengajuan_kgb_for_pkp(
    conn: &MysqlConnection,
) -> Result<Option<Vec<String>>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;
    let result = ren_peg_last
        .select(niplama)
        .filter(namaunit.not_like("Perwakilan%"))
        .filter(namaunit.not_like("Pusat%"))
        .filter(status.eq("Aktif")).load::<String>(conn).optional()?;

    Ok(result)
}

pub fn get_subordinates_open_unpaid_leave_request(
    conn: &MysqlConnection,
    employee_long_id: &str,
) -> Result<Option<Vec<models::UnpaidLeave>>, diesel::result::Error> {
    use crate::schema::t_izin::dsl::*;
    let result = t_izin
        .order(id.desc())
        .filter(
            kd_kat_alasan
                .eq(1)
                .and(kd_proses.eq(0))
                .and(kd_atasanlangsung.eq(employee_long_id))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            kd_kat_alasan
                .eq(2)
                .and(kd_proses.eq(0))
                .and(kd_atasanlangsung.eq(employee_long_id))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            kd_kat_alasan
                .eq(2)
                .and(kd_proses.eq(1))
                .and(kd_atasanlangsung2.eq(employee_long_id))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            kd_kat_alasan
                .eq(3)
                .and(kd_proses.eq(0))
                .and(kd_atasanlangsung.eq(employee_long_id))
                .and(deleted_at.is_null()),
        )
        .or_filter(
            kd_kat_alasan
                .eq(3)
                .and(kd_proses.eq(1))
                .and(kd_atasanlangsung3.eq(employee_long_id))
                .and(deleted_at.is_null()),
        )
        .load::<models::UnpaidLeave>(conn)
        .optional()?;
    Ok(result)
}

pub fn current_leave(
    conn: &MysqlConnection,
    minus: i32,
    employee_id: &str,
) -> Result<Option<models::LeaveAllowance>, diesel::result::Error> {
    use crate::schema::t_saldo_cuti::dsl::*;
    let today = Local::today();
    let result_year = today.year() - minus;
    let result = t_saldo_cuti
        .order(id.desc())
        .filter(niplama_pegawai.eq(employee_id))
        .filter(kode_cuti.ne("103"))
        .filter(tahun.eq(result_year))
        .first::<models::LeaveAllowance>(conn)
        .optional()?;

    Ok(result)
}

pub fn latest_leave(
    conn: &MysqlConnection,
    minus: i32,
    employee_id: &str,
    leave_code: &str,
) -> Result<Option<models::LeaveAllowance>, diesel::result::Error> {
    use crate::schema::t_saldo_cuti::dsl::*;
    let today = Local::today();
    let result_year = today.year() - minus;
    let result = t_saldo_cuti
        .order(id.desc())
        .filter(niplama_pegawai.eq(employee_id))
        .filter(kode_cuti.eq(leave_code))
        .filter(tahun.eq(result_year))
        .first::<models::LeaveAllowance>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_work_leave_by_employee_id(
    conn: &MysqlConnection,
    employee_id: &str,
) -> Result<Option<Vec<models::WorkLeave>>, diesel::result::Error> {
    use crate::schema::t_cuti::dsl::*;
    let result = t_cuti
        .order(id.desc())
        .filter(niplama_pegawai.eq(employee_id))
        .load::<models::WorkLeave>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_latest_mobile_feature(
    conn: &MysqlConnection,
) -> Result<Option<models::MobileFeature>, diesel::result::Error> {
    use crate::schema::mobile_fitur::dsl::*;

    let result = mobile_fitur
        .order(id.desc())
        .first::<models::MobileFeature>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_first_mobile_feature_by_status(
    conn: &MysqlConnection,
    status_param: i32,
) -> Result<Option<models::MobileFeature>, diesel::result::Error> {
    use crate::schema::mobile_fitur::dsl::*;

    let result = mobile_fitur
        .filter(status.eq(&status_param))
        .first::<models::MobileFeature>(conn)
        .optional()?;
    Ok(result)
}

pub fn update_attendance_record(
    conn: &MysqlConnection,
    update_id: i32,
    new_record: FingerprintUpdate,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;
    diesel::update(t_fingerprint_android.filter(id.eq(update_id)))
        .set(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn update_attendance_thl_record(
    conn: &MysqlConnection,
    update_id: i32,
    new_record: FingerprintTHLUpdate,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_fingerprint_thl::dsl::*;
    diesel::update(t_fingerprint_thl.filter(id.eq(update_id)))
        .set(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn create_attendance_record(
    conn: &MysqlConnection,
    new_record: FingerprintCreate,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    diesel::insert_into(t_fingerprint_android)
        .values(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn create_attendance_thl_record(
    conn: &MysqlConnection,
    new_record: FingerprintTHLCreate,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_fingerprint_thl::dsl::*;

    diesel::insert_into(t_fingerprint_thl)
        .values(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

// pub fn get_latest_activity_by_employee_id(
//     conn: &MysqlConnection,
//     offset: i64,
//     employee_id: &str,
// ) -> Result<Option<models::Activity>, diesel::result::Error> {
//     use crate::schema::kinerja_t_usulan_aktivitas::dsl::*;
//     // let today = Local::today();
//     let now = Utc::now();
//     let hari_ini = now.checked_add_signed(Duration::hours(offset)).unwrap().date().naive_utc();
//     let result = kinerja_t_usulan_aktivitas
//         .filter(niplama.eq(employee_id))
//         .filter(tanggal_aktivitas.eq(hari_ini))
//         .first::<models::Activity>(conn)
//         .optional()?;
//     Ok(result)
// }

// pub fn get_latest_activity_by_employee_id_and_date(
//     conn: &MysqlConnection,
//     offset: i64,
//     tanggal: NaiveDate,
//     employee_id: &str,
// ) -> Result<Option<models::Activity>, diesel::result::Error> {
//     use crate::schema::kinerja_t_usulan_aktivitas::dsl::*;
//     // let today = Local::today();
//     let now = Utc::now();
//     let hari_ini = tanggal.checked_add_signed(Duration::hours(offset)).unwrap();
//     let result = kinerja_t_usulan_aktivitas
//         .filter(niplama.eq(employee_id))
//         .filter(tanggal_aktivitas.eq(hari_ini))
//         .first::<models::Activity>(conn)
//         .optional()?;
//     Ok(result)
// }

pub fn get_unitkerja_by_kode_unit(
    conn: &MysqlConnection,
    kode_unit: &str,
) -> Result<Option<models::UnitKerja>, diesel::result::Error> {
    use crate::schema::ref_unitkerja::dsl::*;

    let result = ref_unitkerja
        .filter(key_sort_unit.eq(kode_unit))
        .first::<models::UnitKerja>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_referensi_transport(
    conn: &MysqlConnection,
) -> Result<Option<Vec<models::ReferensiTransport>>, diesel::result::Error> {
    use crate::schema::pemetaan_transportasi_ref::dsl::*;

    let result = pemetaan_transportasi_ref
        .load::<models::ReferensiTransport>(conn)
        .optional()?;
    Ok(result)
}

pub fn get_transport_by_niplama(
    conn: &MysqlConnection,
    employee_id: String
) -> Result<Option<Vec<models::Transportasi>>, diesel::result::Error> {
    use crate::schema::pemetaan_transportasi_t::dsl::*;

    let result = sql_query("select pem.*, ref.jenis_transportasi, ref.nama_transportasi, ref.risiko, kota.s_nama_kota from pemetaan_transportasi_t pem
    join pemetaan_transportasi_ref ref on pem.id_kendaraan = ref.id
    join wm_kota_kab kota on pem.id_kota = kota.s_kd_kota where pem.niplama = ?")
        .bind::<Text, _>(employee_id)
        .get_results(conn).optional()?;
    Ok(result)
}

pub fn get_all_transport(
    conn: &MysqlConnection
) -> Result<Option<Vec<models::Transportasi>>, diesel::result::Error> {
    use crate::schema::pemetaan_transportasi_t::dsl::*;

    let result = sql_query("select pem.*, ref.jenis_transportasi, ref.nama_transportasi, ref.risiko, kota.s_nama_kota from pemetaan_transportasi_t pem
    join pemetaan_transportasi_ref ref on pem.id_kendaraan = ref.id
    join wm_kota_kab kota on pem.id_kota = kota.s_kd_kota")
        .get_results(conn).optional()?;
    Ok(result)
}

pub fn remove_old_transport(
    conn: &MysqlConnection,
    employee_id: String
) -> Result<(), diesel::result::Error> {
    use crate::schema::pemetaan_transportasi_t::dsl::*;
    diesel::delete(pemetaan_transportasi_t.filter(niplama.eq(employee_id))).execute(conn)?;
    Ok(())
}

pub fn create_transport(
    conn: &MysqlConnection,
    new_record: models::TransportBaru
) -> Result<(), diesel::result::Error> {
    use crate::schema::pemetaan_transportasi_t::dsl::*;

    diesel::insert_into(pemetaan_transportasi_t)
        .values(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn create_covid(
    conn: &MysqlConnection,
    new_record: models::CovidBaru
) -> Result<(), diesel::result::Error> {
    use crate::schema::cov_isian::dsl::*;

    diesel::insert_into(cov_isian)
        .values(&new_record)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_latest_covid_by_niplama(
    conn: &MysqlConnection,
    employee_id: String
)-> Result<Option<models::Covid>, diesel::result::Error> {
    use crate::schema::cov_isian::dsl::*;

    let result = cov_isian
        .filter(nip.eq(employee_id))
        .order_by(id_covid.desc())
        .first::<models::Covid>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_all_covid_by_niplama(
    conn: &MysqlConnection,
    employee_id: String
)-> Result<Option<Vec<models::Covid>>, diesel::result::Error> {
    use crate::schema::cov_isian::dsl::*;

    let result = cov_isian
        .filter(nip.eq(employee_id))
        .order_by(id_covid.desc())
        .load::<models::Covid>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_all_covid(
    conn: &MysqlConnection,
)-> Result<Option<Vec<models::Covid>>, diesel::result::Error> {
    use crate::schema::cov_isian::dsl::*;

    let result = cov_isian
        .order_by(id_covid.desc())
        .load::<models::Covid>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_referensi_status_covid(
    conn: &MysqlConnection,
) -> Result<Option<Vec<models::StatusCovid>>, diesel::result::Error> {
    use crate::schema::ref_stat_cov::dsl::*;

    let result = ref_stat_cov
        .load::<models::StatusCovid>(conn)
        .optional()?;

    Ok(result)
}

pub fn get_referensi_gejala_covid(
    conn: &MysqlConnection,
) -> Result<Option<Vec<models::GejalaCovid>>, diesel::result::Error> {
    use crate::schema::ref_gejala_cov::dsl::*;

    let result = ref_gejala_cov
        .load::<models::GejalaCovid>(conn)
        .optional()?;

    Ok(result)
}

pub fn create_backlog(
    conn: &MysqlConnection,
    request: BacklogInsert,
    nip_lama:  Option<String>,
) -> Result<BacklogInsert, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    diesel::insert_into(t_backlog_harian)
        .values(&request)
        .execute(conn)
        .unwrap();

    let hasil = request;
    Ok(hasil)
}

pub fn get_all_backlog_v1(
    conn: &MysqlConnection,
)-> Result<Vec<models::Backlog>, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    let result = t_backlog_harian
        .filter(deleted_at.is_null())
        .order_by(total_skor.desc())
        //.load::<models::Backlog>(conn)?;
        .select((
            id,
            nip_lama,
            nama_kerjaan,
            skor_urgensi,
            urjab_id,
            tgl_mulai,
            tgl_selesai,
            status,
            total_skor,
            skp_id,
            sasaran_id,
            kode_unit,
            skor_prediktabilitas,
            deleted_at,
        ))
        .get_results(conn)?;
    Ok(result)
}

pub fn get_all_backlog(
    conn: &MysqlConnection,
)-> Result<Vec<models::BacklogV2>, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    let result = t_backlog_harian
    .filter(deleted_at.is_null())
    .order_by(total_skor.desc())
    .select((
        id,
        nip_lama,
        sql("(select nama from ren_peg_last rpl where niplama = nip_lama) as nama"),
        sql("(select nip from ren_peg_last rpl where niplama = nip_lama) as nip"),
        nama_kerjaan,
        skor_urgensi,
        urjab_id,
        tgl_mulai,
        tgl_selesai,
        status,
        total_skor,
        skp_id,
        sasaran_id,
        kode_unit,
        skor_prediktabilitas,
        deleted_at
    ))
    .get_results(conn)?;
    Ok(result)
}

pub fn get_all_backlog_by_niplama(
    conn: &MysqlConnection,
    niplama: String,
)-> Result<Vec<models::BacklogV2>, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    let result = t_backlog_harian
    .filter(deleted_at.is_null())
    .filter(tgl_selesai.is_null())
    .filter(status.eq(0))
    .filter(nip_lama.eq(niplama))
    .order_by(total_skor.desc())
    .select((
        id,
        nip_lama,
        sql("(select nama from ren_peg_last rpl where niplama = nip_lama) as nama"),
        sql("(select nip from ren_peg_last rpl where niplama = nip_lama) as nip"),
        nama_kerjaan,
        skor_urgensi,
        urjab_id,
        tgl_mulai,
        tgl_selesai,
        status,
        total_skor,
        skp_id,
        sasaran_id,
        kode_unit,
        skor_prediktabilitas,
        deleted_at
    ))
    .get_results(conn)?;
    Ok(result)
}

pub fn get_backlog_by_id(
    conn: &MysqlConnection,
    id_backlog: i32,
)-> Result<Option<models::BacklogV2>, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    let result = t_backlog_harian
    .filter(deleted_at.is_null())
    .filter(tgl_selesai.is_null())
    .filter(status.eq(0))
    .filter(id.eq(id_backlog))
    .order_by(total_skor.desc())
    .select((
        id,
        nip_lama,
        sql("(select nama from ren_peg_last rpl where niplama = nip_lama) as nama"),
        sql("(select nip from ren_peg_last rpl where niplama = nip_lama) as nip"),
        nama_kerjaan,
        skor_urgensi,
        urjab_id,
        tgl_mulai,
        tgl_selesai,
        status,
        total_skor,
        skp_id,
        sasaran_id,
        kode_unit,
        skor_prediktabilitas,
        deleted_at
    ))
    .get_result(conn).optional()?;
    Ok(result)
}

pub fn post_backlog_update(
    conn: &MysqlConnection,
    request: BacklogUpdate,
    id_backlog:  i32,
) -> Result<(BacklogUpdate), diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;
    diesel::update(t_backlog_harian.filter(id.eq(id_backlog)))
        .set(&request)
        .execute(conn)
        .unwrap();
    let hasil =  request;
    Ok(hasil)
}

pub fn remove_backlog(
    conn: &MysqlConnection,
    delete_at: BacklogDelete,
    id_b:  i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;
    diesel::update(t_backlog_harian.filter(id.eq(id_b)))
        .set(&delete_at)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_all_userorg_v1(
    conn: &MysqlConnection,
)-> Result<Vec<models::Userorg>, diesel::result::Error> {
    use crate::schema::t_userorg::dsl::*;

    let result = t_userorg
        .load::<models::Userorg>(conn)?;
    Ok(result)
}

pub fn get_all_userorg(
    conn: &MysqlConnection,
)-> Result<Vec<models::Userorg>, diesel::result::Error> {
    use crate::schema::t_userorg::dsl::*;

    let result = t_userorg
    .select((
        id,
        pegawai_id,
        kode_atasan,
        kode_atasan1,
        kode_atasan2,
    ))
    .get_results(conn)?;
    Ok(result)
}

pub fn get_pegawai_by_niplama(
    conn: &MysqlConnection,
    nip_lama: String,
) -> Result<Option<String>, diesel::result::Error> {
    use crate::schema::ren_peg_last::dsl::*;

    let result = ren_peg_last
        .select(s_kd_jabdetail )
        .filter(niplama.eq(nip_lama))
        .get_result::<Option<String>>(conn)?;
        //.optional()?;
    Ok(result)
}

pub fn get_atasan_by_kode_jab(
    conn: &MysqlConnection,
    kode_jabatan: String,
)-> Result<(Vec<UserorgV2>), diesel::result::Error> {
    use crate::schema::t_userorg::dsl::*;

    let result = t_userorg
        .filter(kode_atasan.eq(kode_jabatan.clone())
            .or(kode_atasan1.eq(kode_jabatan.clone()))
            .or(kode_atasan2.eq(kode_jabatan.clone())))
        .select((
            id,
            pegawai_id,
            kode_atasan,
            kode_atasan1,
            kode_atasan2,
           // sql(format!("select ... where niplama = {}...", nip_lama))
            //sql(format!"(select s_kd_jabdetail from ren_peg_last rpl where niplama = {}) as s_kd_jabdetail"),
        ))
        .get_results(conn)?;
    Ok(result)
}

pub fn get_attendance_record_v4(
    conn: &MysqlConnection,
    offset: i64,
    niplama_param: String,
    tanggal: NaiveDate,
) -> Result<Option<models::DataKehadiranV4>, diesel::result::Error> {
    use crate::schema::t_fingerprint_android::dsl::*;

    let wektu = NaiveDate::from_ymd(tanggal.year(), tanggal.month(), tanggal.day())
        .and_hms(0, 0, 0)
        .checked_sub_signed(Duration::hours(offset))
        .unwrap()
        .to_owned();
    let wektu_sesuk = wektu
        .clone()
        .checked_add_signed(Duration::days(1))
        .unwrap()
        .to_owned();

    let result = t_fingerprint_android
        .order(id.asc())
        .filter(niplama.eq(niplama_param))
        .filter(created_at.ge(wektu))
        .filter(created_at.le(wektu_sesuk))
        .select((niplama, mode_presensi, datang, lat_datang, long_datang, sumber_datang, pulang, lat_pulang, long_pulang, sumber_pulang, 
            is_fake_gps, is_tampered_timezone, gmt_adjustment,
        sql("(SELECT nipbaru from ren_peg_last peg where peg.niplama = t_fingerprint_android.niplama) nipbaru"),
        sql("(SELECT nip from ren_peg_last peg where peg.niplama = t_fingerprint_android.niplama) nip"),
        sql("(SELECT nama from ren_peg_last peg where peg.niplama = t_fingerprint_android.niplama) nama"),
        sql("(SELECT nama_aktivitas FROM kinerja_t_usulan_aktivitas akt WHERE akt.niplama = t_fingerprint_android.niplama AND date(akt.created_at) = date(now()) LIMIT 1) aktivitas"),
        sql("(SELECT ci.id_stat_cov FROM cov_isian ci WHERE ci.nip = nip AND date(ci.tanggal) = date(now()) LIMIT 1) status_kesehatan"),
        sql("(SELECT judul_cuti from t_cuti_bersama where date(tanggal) = date(now())) libur")
        ))
        .get_result(conn).optional()?;

    Ok(result)
}

pub fn get_atasan_by_niplama(
    conn: &MysqlConnection,
    niplama: String,
)-> Result<(Option<UserorgV2>), diesel::result::Error> {
    use crate::schema::t_userorg::dsl::*;

    let result = t_userorg
    .filter(pegawai_id.eq(niplama))
        .select((
            id,
            pegawai_id,
            kode_atasan,
            kode_atasan1,
            kode_atasan2,
           // sql(format!("select ... where niplama = {}...", nip_lama))
            //sql(format!"(select s_kd_jabdetail from ren_peg_last rpl where niplama = {}) as s_kd_jabdetail"),
        ))
        .get_result(conn).optional()?;
    Ok(result)
}

pub fn create_logging_list(
    conn: &MysqlConnection,
    request: LoggingInsert,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_log_backlog_harian::dsl::*;

    diesel::insert_into(t_log_backlog_harian)
        .values(&request)
        .execute(conn)
        .unwrap();

    let hasil = request;
    Ok(())
}

pub fn get_all_sasaran(
    conn: &MysqlConnection,
)-> Result<Vec<models::Sasaran>, diesel::result::Error> {
    use crate::schema::ref_grup_kinerja::dsl::*;

    let result = ref_grup_kinerja
        .load::<models::Sasaran>(conn)?;
    Ok(result)
}

pub fn get_all_indikator(
    conn: &MysqlConnection,
    id_param: i32,
)-> Result<Vec<models::Indikator>, diesel::result::Error> {
    use crate::schema::ref_sub_group_kinerja::dsl::*;

    let result = ref_sub_group_kinerja
        .filter(id_sasaran.eq(id_param))
        .load::<models::Indikator>(conn)?;
    Ok(result)
}

pub fn create_aktivitas(
    conn: &MysqlConnection,
    request: AktivitasInsert,
) -> Result<AktivitasInsert, diesel::result::Error> {
    use crate::schema::kinerja_t_usulan_aktivitas::dsl::*;

    diesel::insert_into(kinerja_t_usulan_aktivitas)
        .values(&request)
        .execute(conn)
        .unwrap();

    let hasil = request;
    Ok(hasil)
}

pub fn update_backlog_status(
    conn: &MysqlConnection,
    request: BacklogStatus,
    id_b:  i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;
    diesel::update(t_backlog_harian.filter(id.eq(id_b)))
        .set(&request)
        .execute(conn)
        .unwrap();
    Ok(())
}

pub fn get_uraian_jabatan_by_kode(
    conn: &MysqlConnection,
    kode_jabatan: String,
)-> Result<Vec<models::UraianJabatan>, diesel::result::Error> {
    use crate::schema::kinerja_ref_urjab::dsl::*;

    let result = kinerja_ref_urjab
        .filter(s_kd_jabdetail.eq(kode_jabatan))
        .load::<models::UraianJabatan>(conn)?;
    Ok(result)
}

pub fn get_all_project(
    conn: &MysqlConnection,
)-> Result<Vec<models::ProjectBacklog>, diesel::result::Error> {
    use crate::schema::t_backlog_project::dsl::*;

    let result = t_backlog_project
        .load::<models::ProjectBacklog>(conn)?;
    Ok(result)
}

pub fn create_project(
    conn: &MysqlConnection,
    request: ProjectInsert,
) -> Result<ProjectInsert, diesel::result::Error> {
    use crate::schema::t_backlog_project::dsl::*;

    diesel::insert_into(t_backlog_project)
        .values(&request)
        .execute(conn)
        .unwrap();
    let hasil = request;
    Ok(hasil)
}

pub fn get_backlog_by_project(
    conn: &MysqlConnection,
    project_id: i32,
) -> Result<Vec<models::BacklogProject>, diesel::result::Error> {
    use crate::schema::t_backlog_harian::dsl::*;

    let result = t_backlog_harian
        .filter(id_project.eq(project_id))
        .select((
            id,
            nip_lama,
            sql("(select nama from ren_peg_last rpl where niplama = nip_lama) as nama"),
            sql("(select nip from ren_peg_last rpl where niplama = nip_lama) as nip"),
            nama_kerjaan,
            skor_urgensi,
            urjab_id,
            tgl_mulai,
            tgl_selesai,
            status,
            total_skor,
            skp_id,
            sasaran_id,
            kode_unit,
            skor_prediktabilitas,
            id_project,
        ))
        .get_results(conn)?;
    Ok(result)
}

pub fn get_catatan_by_project(
    conn: &MysqlConnection,
    project_id: i32,
) -> Result<Vec<models::CatatanProject>, diesel::result::Error> {
    use crate::schema::t_backlog_catatan::dsl::*;

    let result = t_backlog_catatan
        .filter(id_project.eq(project_id))
        .select((
            id,
            id_project,
            catatan,
            link_lampiran,
        ))
        .get_results(conn)?;
    Ok(result)
}

pub fn create_catatan(
    conn: &MysqlConnection,
    request: CatatanInsert,
    //nip_lama:  Option<String>,
) -> Result<CatatanInsert, diesel::result::Error> {
    use crate::schema::t_backlog_catatan::dsl::*;

    diesel::insert_into(t_backlog_catatan)
        .values(&request)
        .execute(conn)
        .unwrap();

    let hasil = request;
    Ok(hasil)
}

pub fn get_member_by_project(
    conn: &MysqlConnection,
    project_id: i32,
) -> Result<Vec<models::MemberProject>, diesel::result::Error> {
    use crate::schema::t_backlog_project_member::dsl::*;

    let result = t_backlog_project_member
        .filter(id_project.eq(project_id))
        .select((
            id,
            id_project,
            niplama,
            nip,
            nama,
            unitkerja,
            peran
        ))
        //.optional()?;
        .get_results(conn)?;
    Ok(result)
}

pub fn create_member(
    conn: &MysqlConnection,
    request: MemberInsert,
    //nip_lama:  Option<String>,
) -> Result<MemberInsert, diesel::result::Error> {
    use crate::schema::t_backlog_project_member::dsl::*;

    diesel::insert_into(t_backlog_project_member)
        .values(&request)
        .execute(conn)
        .unwrap();

    let hasil = request;
    Ok(hasil)
}