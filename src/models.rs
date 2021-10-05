use crate::schema::cov_isian;
use crate::schema::pemetaan_transportasi_t;
use crate::schema::t_fingerprint_android;
use crate::schema::t_fingerprint_thl;
use crate::schema::t_backlog_harian;
use crate::schema::t_backlog_project;
use crate::schema::t_backlog_catatan;
use crate::schema::t_backlog_project_member;
use crate::schema::t_log_backlog_harian;
use crate::schema::kinerja_t_usulan_aktivitas;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::sql_types::BigInt;
use diesel::sql_types::Bool;
use diesel::sql_types::Datetime;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;
use diesel::sql_types::Varchar;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub username: String,
    pub role_id: i32,
    pub user_nip: Option<String>,
    pub remember_token: Option<String>,
    pub api_token: Option<String>,
    pub firebase_token_map: Option<String>,
    pub firebase_token_exe: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub nomorhp: Option<String>,
    pub aktif: Option<i32>,
    pub key_sort_unit: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct ReferensiTransport {
    pub id: i32,
    pub jenis_transportasi: Option<String>,
    pub nama_transportasi: Option<String>,
    pub risiko: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct KodeAtasan {
    pub id: i32,
    pub pegawai_id: String,
    pub kode_atasan: String,
    pub kode_atasan1: String,
    pub kode_atasan2: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Permission {
    pub id: i32,
    pub modul: String,
    pub fungsi: String,
    pub kriteria: String,
    pub value: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Holiday {
    pub id: i32,
    pub peraturan: Option<String>,
    pub tanggal_peraturan: Option<NaiveDate>,
    pub judul_cuti: Option<String>,
    pub tanggal: Option<NaiveDate>,
    pub tahun: Option<i32>,
    pub ref_unitkerja_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub flag_cuti: Option<i32>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct MobileFeature {
    pub id: i32,
    pub fitur: String,
    pub deskripsi: String,
    pub versi: String,
    pub nama_versi: String,
    pub tanggal_rilis: NaiveDate,
    pub status: i32,
    pub device: String,
    pub urgent: i32,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Fingerprint {
    pub id: i32,
    pub niplama: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub imei: Option<String>,
    pub datang: Option<NaiveTime>,
    pub pulang: Option<NaiveTime>,
    pub status: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_wfh: String,
    pub mode_presensi: Option<i32>,
    pub lat_datang: Option<String>,
    pub long_datang: Option<String>,
    pub lat_pulang: Option<String>,
    pub long_pulang: Option<String>,
    pub alamat_datang: Option<String>,
    pub alamat_pulang: Option<String>,
    pub gmt_adjustment: Option<i64>,
    pub is_fake_gps: Option<bool>,
    pub is_tampered_timezone: Option<bool>,
    pub sumber: i32,
    pub sumber_datang: Option<i32>,
    pub sumber_pulang: Option<i32>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct FingerprintTHL {
    pub id: i32,
    pub nip: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub imei: Option<String>,
    pub datang: Option<NaiveTime>,
    pub pulang: Option<NaiveTime>,
    pub status: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_wfh: String,
    pub mode_presensi: Option<i32>,
    pub lat_datang: Option<String>,
    pub long_datang: Option<String>,
    pub lat_pulang: Option<String>,
    pub long_pulang: Option<String>,
    pub alamat_datang: Option<String>,
    pub alamat_pulang: Option<String>,
    pub gmt_adjustment: Option<i64>,
    pub is_fake_gps: Option<bool>,
    pub is_tampered_timezone: Option<bool>,
    pub sumber: i32,
    pub sumber_datang: Option<i32>,
    pub sumber_pulang: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "t_fingerprint_android"]
pub struct FingerprintUpdate {
    pub niplama: String,
    pub lat: String,
    pub long: String,
    pub imei: String,
    pub datang: NaiveTime,
    pub pulang: NaiveTime,
    pub is_wfh: String,
    pub mode_presensi: i32,
    pub sumber_datang: Option<i32>,
    pub sumber_pulang: Option<i32>,
    pub status: i32,
    pub lat_datang: String,
    pub long_datang: String,
    pub lat_pulang: String,
    pub long_pulang: String,
    pub gmt_adjustment: i64,
    pub is_fake_gps: bool,
    pub is_tampered_timezone: bool,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "t_fingerprint_thl"]
pub struct FingerprintTHLUpdate {
    pub nip: String,
    pub lat: String,
    pub long: String,
    pub imei: String,
    pub datang: NaiveTime,
    pub pulang: NaiveTime,
    pub is_wfh: String,
    pub mode_presensi: i32,
    pub sumber_datang: Option<i32>,
    pub sumber_pulang: Option<i32>,
    pub status: i32,
    pub lat_datang: String,
    pub long_datang: String,
    pub lat_pulang: String,
    pub long_pulang: String,
    pub gmt_adjustment: i64,
    pub is_fake_gps: bool,
    pub is_tampered_timezone: bool,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "t_fingerprint_android"]
pub struct FingerprintCreate {
    pub niplama: String,
    pub lat: String,
    pub long: String,
    pub imei: String,
    pub datang: NaiveTime,
    pub is_wfh: String,
    pub mode_presensi: i32,
    pub sumber_datang: Option<i32>,
    pub status: i32,
    pub lat_datang: String,
    pub long_datang: String,
    pub lat_pulang: String,
    pub long_pulang: String,
    pub gmt_adjustment: i64,
    pub is_fake_gps: bool,
    pub is_tampered_timezone: bool,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "t_fingerprint_thl"]
pub struct FingerprintTHLCreate {
    pub nip: String,
    pub lat: String,
    pub long: String,
    pub imei: String,
    pub datang: NaiveTime,
    pub is_wfh: String,
    pub mode_presensi: i32,
    pub sumber_datang: Option<i32>,
    pub status: i32,
    pub lat_datang: String,
    pub long_datang: String,
    pub lat_pulang: String,
    pub long_pulang: String,
    pub gmt_adjustment: i64,
    pub is_fake_gps: bool,
    pub is_tampered_timezone: bool,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "pemetaan_transportasi_t"]
pub struct TransportBaru {
    pub niplama: String,
    pub id_kota: String,
    pub id_kendaraan: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "cov_isian"]
pub struct CovidBaru {
    pub nip: String,
    pub tanggal: NaiveDate,
    pub id_stat_cov: i32,
    pub id_gejala: String,
    pub gejala1: i32,
    pub gejala2: i32,
    pub gejala3: i32,
    pub gejala4: i32,
    pub gejala5: i32,
    pub gejala6: i32,
    pub gejala7: i32,
    pub gejala8: i32,
    pub gejala9: i32,
    pub keterangan: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Covid {
    pub id: i32,
    pub nip: String,
    pub tanggal: NaiveDate,
    pub id_stat_cov: i32,
    pub id_gejala: String,
    pub gejala1: i32,
    pub gejala2: i32,
    pub gejala3: i32,
    pub gejala4: i32,
    pub gejala5: i32,
    pub gejala6: i32,
    pub gejala7: i32,
    pub gejala8: i32,
    pub gejala9: i32,
    pub keterangan: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct GejalaCovid {
    pub id_gejala: i32,
    pub ur_gejala: String,
    pub singkatan: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct StatusCovid {
    pub id_stat_cov: i32,
    pub ur_stat_cov: String,
    pub singkatan: String,
    pub ket_status: String,
}

#[derive(QueryableByName, Serialize, Debug, Clone)]
pub struct Transportasi {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Nullable<Varchar>"]
    pub niplama: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub id_kota: Option<String>,
    #[sql_type = "Nullable<Integer>"]
    pub id_kendaraan: Option<i32>,
    #[sql_type = "Datetime"]
    pub created_at: NaiveDateTime,
    #[sql_type = "Nullable<Datetime>"]
    pub updated_at: Option<NaiveDateTime>,
    #[sql_type = "Nullable<Varchar>"]
    pub jenis_transportasi: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub nama_transportasi: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub risiko: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub s_nama_kota: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct LeaveAllowance {
    pub id: i32,
    pub niplama_pegawai: Option<String>,
    pub id_cuti: Option<i32>,
    pub kode_cuti: Option<String>,
    pub tahun: Option<i32>,
    pub jml_hari: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub user_create: String,
    pub user_update: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct WorkLeave {
    pub id: i32,
    pub niplama_pegawai: String,
    pub kd_jenis_cuti: Option<String>,
    pub tgl_awal: Option<NaiveDate>,
    pub tgl_akhir: Option<NaiveDate>,
    pub jmlhari: Option<i32>,
    pub alasan: Option<String>,
    pub alamat_pemohon: Option<String>,
    pub kode_unit: Option<String>,
    pub tgl_pengajuan: Option<NaiveDate>,
    pub nip_atasan: Option<String>,
    pub tgl_persetujuan: Option<NaiveDate>,
    pub catatan_persetujuan: Option<String>,
    pub tgl_awal1: Option<NaiveDate>,
    pub tgl_akhir1: Option<NaiveDate>,
    pub no_srt_cuti: Option<String>,
    pub tgl_srt_cuti: Option<NaiveDate>,
    pub notelp_pemohon: Option<String>,
    pub nip_atasan2: Option<String>,
    pub catatan_persetujuan2: Option<String>,
    pub tgl_persetujuan2: Option<NaiveDate>,
    pub tgl_awal2: Option<NaiveDate>,
    pub tgl_akhir2: Option<NaiveDate>,
    pub nip_atasan3: Option<String>,
    pub tgl_persetujuan3: Option<NaiveDate>,
    pub catatan_persetujuan3: Option<String>,
    pub tgl_awal3: Option<NaiveDate>,
    pub tgl_akhir3: Option<NaiveDate>,
    pub nip_persetujuan4: Option<String>,
    pub tgl_persetujuan4: Option<NaiveDateTime>,
    pub catatan_persetujuan4: Option<String>,
    pub kd_proses: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub file_pendukung1: Option<String>,
    pub file_pendukung2: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub stat_plh: Option<i32>,
    pub stat_plh2: Option<i32>,
    pub stat_plh3: Option<i32>,
    pub catatan_hapus: Option<String>,
    pub kode_jabatan_atasan: Option<String>,
    pub kode_jabatan_atasan1: Option<String>,
    pub kode_jabatan_atasan2: Option<String>,
    pub user_create: String,
    pub user_update: String,
    pub tgl_awal_penangguhan: Option<NaiveDate>,
    pub tgl_akhir_penangguhan: Option<NaiveDate>,
    pub catatan_penangguhan: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Activity {
    pub id: i32,
    pub niplama: Option<String>,
    pub nama_aktivitas: Option<String>,
    pub key_sort_unit: Option<String>,
    pub tanggal_aktivitas: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UnpaidLeave {
    pub id: i32,
    pub niplama_pegawai: Option<String>,
    pub kd_kat_alasan: Option<i32>,
    pub kd_alasan: Option<i32>,
    pub kd_jenisizin: Option<String>,
    pub kode_unit: Option<String>,
    pub keterangan: Option<String>,
    pub kd_atasanlangsung: Option<String>,
    pub kd_atasanlangsung2: Option<String>,
    pub kd_atasanlangsung3: Option<String>,
    pub tgl_pengajuan: Option<NaiveDate>,
    pub tgl_awal: Option<NaiveDate>,
    pub tgl_persetujuan: Option<NaiveDate>,
    pub tgl_persetujuan2: Option<NaiveDate>,
    pub catatan_persetujuan: Option<String>,
    pub catatan_persetujuan2: Option<String>,
    pub waktu_fprint: Option<String>,
    pub lokasi_fprint: Option<String>,
    pub no_srt_izin: Option<String>,
    pub kd_proses: Option<i32>,
    pub file_pendukung_izin: Option<String>,
    pub file_pendukung_izin2: Option<String>,
    pub flag_potongan: Option<String>,
    pub jam_izin: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub stat_plh_atasan: Option<i32>,
    pub stat_plh_atasan2: Option<i32>,
    pub stat_plh_atasan3: Option<i32>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UnitKerja {
    pub id: i32,
    pub unitkerja: String,
    pub utara: String,
    pub timur: String,
    pub selatan: String,
    pub barat: String,
    pub key_sort_unit: String,
    pub zona_waktu: String,
    pub s_kd_kota: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Signature {
    pub id : i32,
    pub id_transaksi : Option<i32>,
    pub id_modul : Option<i32>,
    pub id_dokumen : i32,
    pub niplama : String,
    pub tanggal_dokumen : Option<NaiveDate>,
    pub tgl_update : Option<NaiveDateTime>,
    pub ip_address : Option<String>,
    pub unique_code : Option<String>,
    pub created_at : Option<NaiveDateTime>,
    pub updated_at : NaiveDateTime,
    pub id_ref_status_proses : Option<i32>,
    pub niplama_penanda_tangan : Option<String>,
    pub nik : Option<String>,
    pub key_sort_unit_ttd : Option<String>,
    pub created_at_ttd : Option<NaiveDateTime>,
    pub link_file_signed : Option<String>,
    pub deleted_at : Option<NaiveDateTime>,
    pub jenis_dokumen: Option<String>,
    pub grup_dokumen: Option<String>,
    pub nama: String,
    pub nipbaru: Option<String>,
    pub key_sort_unit: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SignatureWithName {
    pub id : i32,
    pub id_transaksi : Option<i32>,
    pub id_modul : Option<i32>,
    pub id_dokumen : i32,
    pub tanggal_dokumen : Option<NaiveDate>,
    pub tgl_update : Option<NaiveDateTime>,
    pub ip_address : Option<String>,
    pub unique_code : Option<String>,
    pub created_at : Option<NaiveDateTime>,
    pub updated_at : NaiveDateTime,
    pub id_ref_status_proses : Option<i32>,
    pub nik : Option<String>,
    pub key_sort_unit_ttd : Option<String>,
    pub created_at_ttd : Option<NaiveDateTime>,
    pub link_file_signed : Option<String>,
    pub deleted_at : Option<NaiveDateTime>,
    pub jenis_dokumen: Option<String>,
    pub grup_dokumen: Option<String>,
    pub nama: String,
    pub nipbaru: Option<String>,
    pub key_sort_unit: Option<String>,
    pub nama_penandatangan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Pegawai {
    pub id: i32,
    pub niplama: String,
    pub nipbaru: Option<String>,
    pub nip: Option<String>,
    pub nama: String,
    pub gelar_depan: Option<String>,
    pub gelar_belakang: Option<String>,
    pub s_nama_lengkap: Option<String>,
    pub carinama_nip: Option<String>,
    pub s_tempat_lahir: String,
    pub d_tgl_lahir: NaiveDate,
    pub jenis_kelamin: String,
    pub s_nama_agama: Option<String>,
    pub usia: i32,
    pub tmtpensiun: Option<NaiveDate>,
    pub golruang: String,
    pub s_nama_pangkat: Option<String>,
    pub d_tmt_sk: Option<NaiveDate>,
    pub lamath_kp: Option<i32>,
    pub lamabl_kp: Option<i32>,
    pub jabatan: Option<String>,
    pub s_nmjabdetailskt: Option<String>,
    pub tmt_jab: Option<NaiveDate>,
    pub s_kd_jabdetail: Option<String>,
    pub jenis_jab: String,
    pub jenis_jab_group: Option<String>,
    pub lamath_jab: Option<i32>,
    pub lamabl_jab: Option<i32>,
    pub peran: Option<String>,
    pub karpeg: Option<String>,
    pub s_nama_instansiunitorg: Option<String>,
    pub s_skt_instansiunitorg: Option<String>,
    pub namaunit_lengkap: Option<String>,
    pub tmt_unit: Option<NaiveDate>,
    pub s_kd_instansiunitorg: Option<String>,
    pub s_kd_instansiunitkerjal1: Option<String>,
    pub s_kd_instansiunitkerjal2: Option<String>,
    pub lamath_unit: Option<i32>,
    pub lamabl_unit: Option<i32>,
    pub namaunit: String,
    pub key_sort_unit: Option<String>,
    pub s_skt_pendidikan: Option<String>,
    pub s_nama_strata_skt: Option<String>,
    pub s_nama_fakultasbidang: Option<String>,
    pub s_nama_jurusan: Option<String>,
    pub d_tgl_lulus: Option<NaiveDate>,
    pub total_pak: Option<f64>,
    pub s_no_sk_kgb: Option<String>,
    pub i_maker_th_kgb: Option<i32>,
    pub i_maker_bl_kgb: Option<i32>,
    pub d_tmt_kgb: Option<NaiveDate>,
    pub s_nama_diklatfung: Option<String>,
    pub s_nosert_diklatfung: Option<String>,
    pub d_tglser_diklatfung: Option<NaiveDate>,
    pub diklat_struk: Option<String>,
    pub s_nosert_diklatstruk: Option<String>,
    pub d_tglser_diklatstruk: Option<NaiveDate>,
    pub nama_pasangan: Option<String>,
    pub unit_pasangan: Option<String>,
    pub s_alamat: Option<String>,
    pub status: String,
    pub id_sort: i32,
    pub tgl_update: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct PegawaiTHL {
    pub id: i32,
    pub user_id: String,
    pub nipbaru: Option<String>,
    pub nip: Option<String>,
    pub nama: String,
    pub gelar_depan: Option<String>,
    pub gelar_belakang: Option<String>,
    pub s_nama_lengkap: Option<String>,
    pub carinama_nip: Option<String>,
    pub s_tempat_lahir: String,
    pub d_tgl_lahir: NaiveDate,
    pub jenis_kelamin: String,
    pub s_nama_agama: Option<String>,
    pub usia: i32,
    pub tmtpensiun: Option<NaiveDate>,
    pub golruang: Option<String>,
    pub s_nama_pangkat: Option<String>,
    pub d_tmt_sk: Option<NaiveDate>,
    pub lamath_kp: Option<i32>,
    pub lamabl_kp: Option<i32>,
    pub jabatan: Option<String>,
    pub s_nmjabdetailskt: Option<String>,
    pub tmt_jab: Option<NaiveDate>,
    pub s_kd_jabdetail: Option<String>,
    pub jenis_jab_thl: Option<String>,
    pub jenis_jab_group_thl: Option<String>,
    pub lamath_jab: Option<i32>,
    pub lamabl_jab: Option<i32>,
    pub peran: Option<String>,
    pub karpeg: Option<String>,
    pub s_nama_instansiunitorg: Option<String>,
    pub s_skt_instansiunitorg: Option<String>,
    pub namaunit_lengkap: Option<String>,
    pub tmt_unit: Option<NaiveDate>,
    pub s_kd_instansiunitorg: Option<String>,
    pub s_kd_instansiunitkerjal1: Option<String>,
    pub s_kd_instansiunitkerjal2: Option<String>,
    pub lamath_unit: Option<i32>,
    pub lamabl_unit: Option<i32>,
    pub namaunit: Option<String>,
    pub key_sort_unit: Option<String>,
    pub s_skt_pendidikan: Option<String>,
    pub s_nama_strata_skt: Option<String>,
    pub s_nama_fakultasbidang: Option<String>,
    pub s_nama_jurusan: Option<String>,
    pub d_tgl_lulus: Option<NaiveDate>,
    pub total_pak: Option<f64>,
    pub s_no_sk_kgb: Option<String>,
    pub i_maker_th_kgb: Option<i32>,
    pub i_maker_bl_kgb: Option<i32>,
    pub d_tmt_kgb: Option<NaiveDate>,
    pub s_nama_diklatfung: Option<String>,
    pub s_nosert_diklatfung: Option<String>,
    pub d_tglser_diklatfung: Option<NaiveDate>,
    pub diklat_struk: Option<String>,
    pub s_nosert_diklatstruk: Option<String>,
    pub d_tglser_diklatstruk: Option<NaiveDate>,
    pub nama_pasangan: Option<String>,
    pub unit_pasangan: Option<String>,
    pub s_alamat: Option<String>,
    pub status: String,
    pub id_sort: i32,
    pub tgl_update: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Backlog {
    pub id: i32,
    pub nip_lama: Option<String>,
    pub nama_kerjaan: Option<String>,
    pub skor_urgensi: Option<i32>,
    pub urjab_id: Option<i32>,
    pub tgl_mulai: Option<NaiveDate>,
    pub tgl_selesai: Option<NaiveDate>,
    pub status: Option<i32>,
    pub total_skor: Option<i32>,
    pub skp_id: Option<i32>,
    pub sasaran_id: Option<i32>,
    pub kode_unit: Option<String>,
    pub skor_prediktabilitas: Option<i32>,
    pub deleted_at: Option<NaiveDateTime>
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct BacklogV2 {
    pub id: i32,
    pub nip_lama: Option<String>,
    pub nama: Option<String>,
    pub nip: Option<String>,
    pub nama_kerjaan: Option<String>,
    pub skor_urgensi: Option<i32>,
    pub urjab_id: Option<i32>,
    pub tgl_mulai: Option<NaiveDate>,
    pub tgl_selesai: Option<NaiveDate>,
    pub status: Option<i32>,
    pub total_skor: Option<i32>,
    pub skp_id: Option<i32>,
    pub sasaran_id: Option<i32>,
    pub kode_unit: Option<String>,
    pub skor_prediktabilitas: Option<i32>,
    pub deleted_at: Option<NaiveDateTime>
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct BacklogProject {
    pub id: i32,
    pub nip_lama: Option<String>,
    pub nama: Option<String>,
    pub nip: Option<String>,
    pub nama_kerjaan: Option<String>,
    pub skor_urgensi: Option<i32>,
    pub urjab_id: Option<i32>,
    pub tgl_mulai: Option<NaiveDate>,
    pub tgl_selesai: Option<NaiveDate>,
    pub status: Option<i32>,
    pub total_skor: Option<i32>,
    pub skp_id: Option<i32>,
    pub sasaran_id: Option<i32>,
    pub kode_unit: Option<String>,
    pub skor_prediktabilitas: Option<i32>,
    pub id_project: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "t_backlog_harian"]
pub struct BacklogInsert {
    pub nip_lama: Option<String>,
    pub nama_kerjaan: String,
    pub skor_urgensi: i32,
    pub urjab_id: i32,
    pub tgl_mulai: NaiveDate,
    pub tgl_selesai: Option<NaiveDate>,
    pub status: Option<i32>,
    pub total_skor: i32,
    pub skp_id: i32,
    pub kode_unit: String,
    pub sasaran_id: i32,
    pub skor_prediktabilitas: i32,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[table_name = "t_backlog_harian"]
pub struct BacklogUpdate {
    // pub nip_lama: Option<String>,
    pub nama_kerjaan: String,
    pub skor_urgensi: i32,
    pub urjab_id: i32,
    pub tgl_mulai: NaiveDate,
    pub tgl_selesai: Option<NaiveDate>,
    // pub status: i32,
    pub total_skor: i32,
    pub skp_id: i32,
    pub skor_prediktabilitas: i32,
    pub sasaran_id: i32,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[table_name = "t_backlog_harian"]
pub struct BacklogDelete {
    pub deleted_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[table_name = "t_backlog_harian"]
pub struct BacklogStatus {
    pub status: i32,
    pub tgl_selesai: NaiveDate,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Userorg {
    pub id: i32,
    pub pegawai_id: Option<String>,
    pub kode_atasan: Option<String>,
    pub kode_atasan1: Option<String>,
    pub kode_atasan2: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UserorgV2 {
    pub id: i32,
    pub pegawai_id: Option<String>,
    pub kode_atasan: Option<String>,
    pub kode_atasan1: Option<String>,
    pub kode_atasan2: Option<String>,
    //pub s_kd_jabdetail: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "t_log_backlog_harian"]
pub struct LoggingInsert {
    pub backlog_id: Option<i32>,
    pub keterangan: Option<String>,
    pub nip_lama: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "kinerja_t_usulan_aktivitas"]
pub struct AktivitasInsert {
    pub niplama: Option<String>,
    pub nama_aktivitas: Option<String>,
    pub key_sort_unit: Option<String>,
    pub tanggal_aktivitas: NaiveDate,
    pub id_sasaran: i32,
    pub id_sub_sasaran: i32,
    pub status_wfh: i32,
    pub atasan1: String,
    pub atasan2: String,
    pub atasan3: String,
    pub niplama_atasan: Option<String>,
    pub nama_atasan: Option<String>,
    pub jabatan_atasan: Option<String>,
    pub namaunit_atasan: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub sumber_lokasi: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct DataKehadiranV4 {
    pub niplama: Option<String>,
    pub mode_presensi: Option<i32>,
    pub datang: Option<NaiveTime>,
    pub lat_datang: Option<String>,
    pub long_datang: Option<String>,
    pub sumber_datang: Option<i32>,
    pub pulang: Option<NaiveTime>,
    pub lat_pulang: Option<String>,
    pub long_pulang: Option<String>,
    pub sumber_pulang: Option<i32>,
    pub is_fake_gps: Option<bool>,
    pub is_tampered_timezone: Option<bool>,
    pub gmt_adjustment: Option<i64>,
    pub nipbaru: Option<String>,
    pub nip: Option<String>,
    pub nama: Option<String>,
    pub aktivitas: Option<String>,
    pub status_kesehatan: Option<i32>,
    pub libur: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct AktivitasInsertV2 {
    pub niplama: Option<String>,
    pub nama_aktivitas: Option<String>,
    pub key_sort_unit: Option<String>,
    pub tanggal_aktivitas: NaiveDate,
    pub id_b: Option<i32>,
    pub id_sasaran: i32,
    pub id_sub_sasaran: i32,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Sasaran {
    pub id_sasaran: i32,
    pub nama_sasaran_kinerja: Option<String>,
    pub nama_kategori_1: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Indikator {
    pub id_sub_sasaran: i32,
    pub id_sasaran: i32,
    pub nama_indikator: Option<String>,
    pub nama_kategori_2: Option<String>,
    pub lingkup: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct UraianJabatan {
    pub id: i32,
    pub rskp_kegiatan_pk:  Option<i32>,
    pub rjabatan_fk: Option<i32>,
    pub s_kd_jabdetail: Option<String>,
    pub nama_sasaran: Option<String>,
    pub target_output: Option<String>,
    pub target_kualitas: Option<String>,
    pub target_waktu: Option<i32>,
    pub target_biaya: Option<String>,
    pub jenis_sasaran: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct ProjectBacklog {
    pub id: i32,
    pub judul: String,
    pub tgl_mulai: NaiveDate,
    pub tgl_selesai: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "t_backlog_project"]
pub struct ProjectInsert {
    pub judul: String,
    pub tgl_mulai: NaiveDate,
    pub tgl_selesai: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct CatatanProject {
    pub id: i32,
    pub id_project: i32,
    pub catatan: Option<String>,
    pub link_lampiran: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "t_backlog_catatan"]
pub struct CatatanInsert {
    pub id_project: i32,
    pub catatan: Option<String>,
    pub link_lampiran: Option<String>,
}

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct MemberProject {
    pub id: i32,
    pub id_project: i32,
    pub niplama: String,
    pub nip: String,
    pub nama: String,
    pub unitkerja: String,
    pub peran: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "t_backlog_project_member"]
pub struct MemberInsert {
    pub id_project: i32,
    pub niplama: String,
    pub nip: String,
    pub nama: String,
    pub unitkerja: String,
    pub peran: String,
}