allow_tables_to_appear_in_same_query!(ren_peg_last, t_userorg);

table! {
    ren_peg_last(id) {
        id -> Integer,
        niplama -> Varchar,
        nipbaru -> Nullable<Varchar>,
        nip -> Nullable<Varchar>,
        nama -> Varchar,
        gelar_depan -> Nullable<Varchar>,
        gelar_belakang -> Nullable<Varchar>,
        s_nama_lengkap -> Nullable<Varchar>,
        carinama_nip -> Nullable<Varchar>,
        s_tempat_lahir -> Varchar,
        d_tgl_lahir -> Date,
        jenis_kelamin -> Varchar,
        s_nama_agama -> Nullable<Varchar>,
        usia -> Integer,
        tmtpensiun -> Nullable<Date>,
        golruang -> Varchar,
        s_nama_pangkat -> Nullable<Varchar>,
        d_tmt_sk -> Nullable<Date>,
        lamath_kp -> Nullable<Integer>,
        lamabl_kp -> Nullable<Integer>,
        jabatan -> Nullable<Varchar>,
        s_nmjabdetailskt -> Nullable<Varchar>,
        tmt_jab -> Nullable<Date>,
        s_kd_jabdetail -> Nullable<Varchar>,
        jenis_jab -> Varchar,
        jenis_jab_group -> Nullable<Varchar>,
        lamath_jab -> Nullable<Integer>,
        lamabl_jab -> Nullable<Integer>,
        peran -> Nullable<Varchar>,
        karpeg -> Nullable<Varchar>,
        s_nama_instansiunitorg -> Nullable<Varchar>,
        s_skt_instansiunitorg -> Nullable<Varchar>,
        namaunit_lengkap -> Nullable<Varchar>,
        tmt_unit -> Nullable<Date>,
        s_kd_instansiunitorg -> Nullable<Varchar>,
        s_kd_instansiunitkerjal1 -> Nullable<Varchar>,
        s_kd_instansiunitkerjal2 -> Nullable<Varchar>,
        lamath_unit -> Nullable<Integer>,
        lamabl_unit -> Nullable<Integer>,
        namaunit -> Varchar,
        key_sort_unit -> Nullable<Varchar>,
        s_skt_pendidikan -> Nullable<Varchar>,
        s_nama_strata_skt -> Nullable<Varchar>,
        s_nama_fakultasbidang -> Nullable<Varchar>,
        s_nama_jurusan -> Nullable<Varchar>,
        d_tgl_lulus -> Nullable<Date>,
        total_pak -> Nullable<Double>,
        // d_tgl_awal -> Nullable<Date>,
        // d_tgl_akhir -> Nullable<Date>,
        s_no_sk_kgb -> Nullable<Varchar>,
        i_maker_th_kgb -> Nullable<Integer>,
        i_maker_bl_kgb -> Nullable<Integer>,
        d_tmt_kgb -> Nullable<Date>,
        s_nama_diklatfung -> Nullable<Varchar>,
        s_nosert_diklatfung -> Nullable<Varchar>,
        d_tglser_diklatfung -> Nullable<Date>,
        diklat_struk -> Nullable<Varchar>,
        s_nosert_diklatstruk -> Nullable<Varchar>,
        d_tglser_diklatstruk -> Nullable<Date>,
        // sert_jfa -> Nullable<Varchar>,
        nama_pasangan -> Nullable<Varchar>,
        unit_pasangan -> Nullable<Varchar>,
        s_alamat -> Nullable<Varchar>,
        // sert_profesi -> Nullable<Varchar>,
        // kel_jab -> Nullable<Varchar>,
        status -> Varchar,
        id_sort -> Integer,
        tgl_update -> Nullable<Datetime>,
    }
}

table! {
    ren_peg_last_thl(id) {
        id -> Integer,
        user_id -> Varchar,
        nipbaru -> Nullable<Varchar>,
        nip -> Nullable<Varchar>,
        nama -> Varchar,
        gelar_depan -> Nullable<Varchar>,
        gelar_belakang -> Nullable<Varchar>,
        s_nama_lengkap -> Nullable<Varchar>,
        carinama_nip -> Nullable<Varchar>,
        s_tempat_lahir -> Varchar,
        d_tgl_lahir -> Date,
        jenis_kelamin -> Varchar,
        s_nama_agama -> Nullable<Varchar>,
        usia -> Integer,
        tmtpensiun -> Nullable<Date>,
        golruang -> Nullable<Varchar>,
        s_nama_pangkat -> Nullable<Varchar>,
        d_tmt_sk -> Nullable<Date>,
        lamath_kp -> Nullable<Integer>,
        lamabl_kp -> Nullable<Integer>,
        jabatan -> Nullable<Varchar>,
        s_nmjabdetailskt -> Nullable<Varchar>,
        tmt_jab -> Nullable<Date>,
        s_kd_jabdetail -> Nullable<Varchar>,
        jenis_jab_thl -> Nullable<Varchar>,
        jenis_jab_group_thl -> Nullable<Varchar>,
        lamath_jab -> Nullable<Integer>,
        lamabl_jab -> Nullable<Integer>,
        peran -> Nullable<Varchar>,
        karpeg -> Nullable<Varchar>,
        s_nama_instansiunitorg -> Nullable<Varchar>,
        s_skt_instansiunitorg -> Nullable<Varchar>,
        namaunit_lengkap -> Nullable<Varchar>,
        tmt_unit -> Nullable<Date>,
        s_kd_instansiunitorg -> Nullable<Varchar>,
        s_kd_instansiunitkerjal1 -> Nullable<Varchar>,
        s_kd_instansiunitkerjal2 -> Nullable<Varchar>,
        lamath_unit -> Nullable<Integer>,
        lamabl_unit -> Nullable<Integer>,
        namaunit -> Nullable<Varchar>,
        key_sort_unit -> Nullable<Varchar>,
        s_skt_pendidikan -> Nullable<Varchar>,
        s_nama_strata_skt -> Nullable<Varchar>,
        s_nama_fakultasbidang -> Nullable<Varchar>,
        s_nama_jurusan -> Nullable<Varchar>,
        d_tgl_lulus -> Nullable<Date>,
        total_pak -> Nullable<Double>,
        // d_tgl_awal -> Nullable<Date>,
        // d_tgl_akhir -> Nullable<Date>,
        s_no_sk_kgb -> Nullable<Varchar>,
        i_maker_th_kgb -> Nullable<Integer>,
        i_maker_bl_kgb -> Nullable<Integer>,
        d_tmt_kgb -> Nullable<Date>,
        s_nama_diklatfung -> Nullable<Varchar>,
        s_nosert_diklatfung -> Nullable<Varchar>,
        d_tglser_diklatfung -> Nullable<Date>,
        diklat_struk -> Nullable<Varchar>,
        s_nosert_diklatstruk -> Nullable<Varchar>,
        d_tglser_diklatstruk -> Nullable<Date>,
        // sert_jfa -> Nullable<Varchar>,
        nama_pasangan -> Nullable<Varchar>,
        unit_pasangan -> Nullable<Varchar>,
        s_alamat -> Nullable<Varchar>,
        // sert_profesi -> Nullable<Varchar>,
        // kel_jab -> Nullable<Varchar>,
        status -> Varchar,
        id_sort -> Integer,
        tgl_update -> Nullable<Datetime>,
    }
}

table! {
    pemetaan_transportasi_ref (id) {
        id -> Integer,
        jenis_transportasi -> Nullable<Varchar>,
        nama_transportasi -> Nullable<Varchar>,
        risiko -> Nullable<Varchar>,
    }
}

table! {
    pemetaan_transportasi_t (id) {
        id -> Integer,
        niplama -> Nullable<Varchar>,
        id_kota -> Nullable<Varchar>,
        id_kendaraan -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    t_cuti (id) {
        id -> Integer,
        niplama_pegawai -> Varchar,
        kd_jenis_cuti -> Nullable<Varchar>,
        tgl_awal -> Nullable<Date>,
        tgl_akhir -> Nullable<Date>,
        jmlhari -> Nullable<Integer>,
        alasan -> Nullable<Text>,
        alamat_pemohon -> Nullable<Varchar>,
        kode_unit -> Nullable<Varchar>,
        tgl_pengajuan -> Nullable<Date>,
        nip_atasan -> Nullable<Varchar>,
        tgl_persetujuan -> Nullable<Date>,
        catatan_persetujuan -> Nullable<Text>,
        tgl_awal1 -> Nullable<Date>,
        tgl_akhir1 -> Nullable<Date>,
        no_srt_cuti -> Nullable<Varchar>,
        tgl_srt_cuti -> Nullable<Date>,
        notelp_pemohon -> Nullable<Varchar>,
        nip_atasan2 -> Nullable<Varchar>,
        catatan_persetujuan2 -> Nullable<Text>,
        tgl_persetujuan2 -> Nullable<Date>,
        tgl_awal2 -> Nullable<Date>,
        tgl_akhir2 -> Nullable<Date>,
        nip_atasan3 -> Nullable<Varchar>,
        tgl_persetujuan3 -> Nullable<Date>,
        catatan_persetujuan3 -> Nullable<Text>,
        tgl_awal3 -> Nullable<Date>,
        tgl_akhir3 -> Nullable<Date>,
        nip_persetujuan4 -> Nullable<Varchar>,
        tgl_persetujuan4 -> Nullable<Timestamp>,
        catatan_persetujuan4 -> Nullable<Varchar>,
        kd_proses -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        file_pendukung1 -> Nullable<Varchar>,
        file_pendukung2 -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
        stat_plh -> Nullable<Integer>,
        stat_plh2 -> Nullable<Integer>,
        stat_plh3 -> Nullable<Integer>,
        catatan_hapus -> Nullable<Text>,
        kode_jabatan_atasan -> Nullable<Varchar>,
        kode_jabatan_atasan1 -> Nullable<Varchar>,
        kode_jabatan_atasan2 -> Nullable<Varchar>,
        user_create -> Varchar,
        user_update -> Varchar,
        tgl_awal_penangguhan -> Nullable<Date>,
        tgl_akhir_penangguhan -> Nullable<Date>,
        catatan_penangguhan -> Nullable<Text>,
    }
}

table! {
    t_cuti_bersama (id) {
        id -> Integer,
        peraturan -> Nullable<Varchar>,
        tanggal_peraturan -> Nullable<Date>,
        judul_cuti -> Nullable<Varchar>,
        tanggal -> Nullable<Date>,
        tahun -> Nullable<Integer>,
        ref_unitkerja_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        flag_cuti -> Nullable<Integer>,
    }
}

table! {
    t_izin (id) {
        id -> Integer,
        niplama_pegawai -> Nullable<Varchar>,
        kd_kat_alasan -> Nullable<Integer>,
        kd_alasan -> Nullable<Integer>,
        kd_jenisizin -> Nullable<Varchar>,
        kode_unit -> Nullable<Varchar>,
        keterangan -> Nullable<Text>,
        kd_atasanlangsung -> Nullable<Varchar>,
        kd_atasanlangsung2 -> Nullable<Varchar>,
        kd_atasanlangsung3 -> Nullable<Varchar>,
        tgl_pengajuan -> Nullable<Date>,
        tgl_awal -> Nullable<Date>,
        // tgl_akhir -> Nullable<Date>,
        tgl_persetujuan -> Nullable<Date>,
        tgl_persetujuan2 -> Nullable<Date>,
        catatan_persetujuan -> Nullable<Text>,
        catatan_persetujuan2 -> Nullable<Text>,
        // tgl_fprint -> Nullable<Date>,
        waktu_fprint -> Nullable<Varchar>,
        lokasi_fprint -> Nullable<Text>,
        // tgl_srt_fprint -> Nullable<Date>,
        no_srt_izin -> Nullable<Varchar>,
        // tgl_srt_izin -> Nullable<Date>,
        kd_proses -> Nullable<Integer>,
        file_pendukung_izin -> Nullable<Varchar>,
        file_pendukung_izin2 -> Nullable<Varchar>,
        // updated_at -> Nullable<Datetime>,
        // created_at -> Nullable<Datetime>,
        flag_potongan -> Nullable<Varchar>,
        jam_izin -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
        stat_plh_atasan -> Nullable<Integer>,
        stat_plh_atasan2 -> Nullable<Integer>,
        stat_plh_atasan3 -> Nullable<Integer>,
    }
}

table! {
    t_saldo_cuti (id) {
        id -> Integer,
        niplama_pegawai -> Nullable<Varchar>,
        id_cuti -> Nullable<Integer>,
        kode_cuti -> Nullable<Varchar>,
        tahun -> Nullable<Integer>,
        jml_hari -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        user_create -> Varchar,
        user_update -> Varchar,
    }
}

table! {
    t_fingerprint_thl (id) {
        id -> Integer,
        nip -> Nullable<Varchar>,
        lat -> Nullable<Varchar>,
        long -> Nullable<Varchar>,
        imei -> Nullable<Varchar>,
        datang -> Nullable<Time>,
        pulang -> Nullable<Time>,
        status -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        is_wfh -> Varchar,
        mode_presensi -> Nullable<Integer>,
        lat_datang -> Nullable<Varchar>,
        long_datang -> Nullable<Varchar>,
        lat_pulang -> Nullable<Varchar>,
        long_pulang -> Nullable<Varchar>,
        alamat_datang -> Nullable<Varchar>,
        alamat_pulang -> Nullable<Varchar>,
        gmt_adjustment -> Nullable<BigInt>,
        is_fake_gps -> Nullable<Bool>,
        is_tampered_timezone -> Nullable<Bool>,
        sumber -> Integer,
        sumber_datang -> Nullable<Integer>,
        sumber_pulang -> Nullable<Integer>,
    }
}

table! {
    t_fingerprint_thl_shift (id) {
        id -> Integer,
        id_fingerprint -> Integer,
        datang -> Nullable<Time>,
        pulang -> Nullable<Time>,
        lat_datang -> Nullable<Varchar>,
        long_datang -> Nullable<Varchar>,
        lat_pulang -> Nullable<Varchar>,
        long_pulang -> Nullable<Varchar>,
        mode_presensi -> Integer,
        keterangan -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        sumber_datang -> Nullable<Integer>,
        sumber_pulang -> Nullable<Integer>,
    }
}

table! {
    t_fingerprint_android (id) {
        id -> Integer,
        niplama -> Nullable<Varchar>,
        lat -> Nullable<Varchar>,
        long -> Nullable<Varchar>,
        imei -> Nullable<Varchar>,
        datang -> Nullable<Time>,
        pulang -> Nullable<Time>,
        status -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        is_wfh -> Varchar,
        mode_presensi -> Nullable<Integer>,
        lat_datang -> Nullable<Varchar>,
        long_datang -> Nullable<Varchar>,
        lat_pulang -> Nullable<Varchar>,
        long_pulang -> Nullable<Varchar>,
        alamat_datang -> Nullable<Varchar>,
        alamat_pulang -> Nullable<Varchar>,
        gmt_adjustment -> Nullable<BigInt>,
        is_fake_gps -> Nullable<Bool>,
        is_tampered_timezone -> Nullable<Bool>,
        sumber -> Integer,
        sumber_datang -> Nullable<Integer>,
        sumber_pulang -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        role_id -> Integer,
        user_nip -> Nullable<Varchar>,
        remember_token -> Nullable<Varchar>,
        api_token -> Nullable<Varchar>,
        firebase_token_map -> Nullable<Varchar>,
        firebase_token_exe -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        nomorhp -> Nullable<Varchar>,
        aktif -> Nullable<Integer>,
        key_sort_unit -> Nullable<Varchar>,
    }
}

table! {
    t_permission (id) {
        id -> Integer,
        modul -> Varchar,
        fungsi -> Varchar,
        kriteria -> Varchar,
        value -> Varchar,
    }
}

table! {
    mobile_fitur (id) {
        id -> Integer,
        fitur -> Varchar,
        deskripsi -> Varchar,
        versi -> Varchar,
        nama_versi -> Varchar,
        tanggal_rilis -> Date,
        status -> Integer,
        device -> Varchar,
        urgent -> Integer,
    }
}

table! {
    ref_unitkerja (id) {
        id -> Integer,
        unitkerja -> Varchar,
        utara -> Varchar,
        timur -> Varchar,
        selatan -> Varchar,
        barat -> Varchar,
        key_sort_unit -> Varchar,
        zona_waktu -> Varchar,
        s_kd_kota -> Nullable<Varchar>,
    }
}

table! {
    cov_isian (id_covid) {
        id_covid -> Integer,
        nip -> Varchar,
        tanggal -> Date,
        id_stat_cov -> Integer,
        id_gejala -> Varchar,
        gejala1 -> Integer,
        gejala2 -> Integer,
        gejala3 -> Integer,
        gejala4 -> Integer,
        gejala5 -> Integer,
        gejala6 -> Integer,
        gejala7 -> Integer,
        gejala8 -> Integer,
        gejala9 -> Integer,
        keterangan -> Text,
        updated_at -> Datetime, 
    }
}

table! {
    ref_gejala_cov (id_gejala) {
        id_gejala -> Integer,
        ur_gejala -> Varchar,
        singkatan -> Varchar,
    }
}

table! {
    ref_stat_cov (id_stat_cov) {
        id_stat_cov -> Integer,
        ur_stat_cov -> Varchar,
        singkatan -> Varchar,
        ket_status -> Text,
    }
}

// Table Digital Signature

joinable!(t_signature_digital -> ref_jenis_dokumen (id_dokumen));
allow_tables_to_appear_in_same_query!(t_signature_digital, ref_jenis_dokumen, ren_peg_last);

table! {
    t_signature_digital (id) {
        id -> Integer,
        id_transaksi -> Nullable<Integer>,
        id_modul -> Nullable<Integer>,
        id_dokumen -> Integer,
        niplama -> Varchar,
        tanggal_dokumen -> Nullable<Date>,
        tgl_update -> Nullable<Datetime>,
        ip_address -> Nullable<Varchar>,
        unique_code -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Datetime,
        id_ref_status_proses -> Nullable<Integer>,
        niplama_penanda_tangan -> Nullable<Varchar>,
        nik -> Nullable<Varchar>,
        key_sort_unit_ttd -> Nullable<Varchar>,
        created_at_ttd -> Nullable<Datetime>,
        link_file_signed -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    ref_jenis_dokumen (id) {
        id -> Integer,
        jenis_dokumen -> Nullable<Varchar>,
        is_aktif -> Nullable<Integer>,
        grup_dokumen -> Nullable<Varchar>,
    }
}

table! {
    t_backlog_harian (id) {
        id -> Integer,
        nip_lama -> Nullable<Varchar>,
        nama_kerjaan -> Nullable<Varchar>,
        skor_urgensi -> Nullable<Integer>,
        urjab_id -> Nullable<Integer>,
        tgl_mulai -> Nullable<Date>,
        tgl_selesai -> Nullable<Date>,
        status -> Nullable<Integer>,
        total_skor -> Nullable<Integer>,
        skp_id -> Nullable<Integer>,
        sasaran_id -> Nullable<Integer>,
        kode_unit -> Nullable<Varchar>,
        skor_prediktabilitas -> Nullable<Integer>,
        deleted_at -> Nullable<Datetime>,
        id_project -> Integer,
    }
}

table! {
    t_userorg (id) {
        id -> Integer,
        pegawai_id -> Nullable<Varchar>,
        kode_atasan -> Nullable<Varchar>,
        kode_atasan1 -> Nullable<Varchar>,
        kode_atasan2 -> Nullable<Varchar>,
    }
}

table! {
    t_log_backlog_harian (id) {
        id -> Integer,
        backlog_id -> Nullable<Integer>,
        keterangan -> Nullable<Varchar>,
        nip_lama -> Nullable<Varchar>,
    }
}

table! {
    kinerja_t_usulan_aktivitas (id) {
        id -> Integer,
        niplama -> Nullable<Varchar>,
        nama_aktivitas -> Nullable<Text>,
        key_sort_unit -> Nullable<Varchar>,
        tanggal_aktivitas -> Date,
        id_sasaran -> Integer,
        id_sub_sasaran -> Integer,
        status_wfh -> Integer,
        atasan1 -> Varchar,
        atasan2 -> Varchar,
        atasan3 -> Varchar,
        niplama_atasan -> Nullable<Varchar>,
        nama_atasan -> Nullable<Varchar>,
        jabatan_atasan -> Nullable<Varchar>,
        namaunit_atasan -> Nullable<Varchar>,
        lat -> Nullable<Varchar>,
        long -> Nullable<Varchar>,
        sumber_lokasi -> Nullable<Varchar>,
    }
}

table! {
    ref_grup_kinerja (id_sasaran) {
        id_sasaran ->Integer,
        nama_sasaran_kinerja -> Nullable<Varchar>,
        nama_kategori_1 -> Nullable<Varchar>,
    }
}

table! {
    ref_sub_group_kinerja (id_sub_sasaran) {
        id_sub_sasaran -> Integer,
        id_sasaran -> Integer,
        nama_indikator -> Nullable<Varchar>,
        nama_kategori_2 -> Nullable<Varchar>,
        lingkup -> Nullable<Varchar>,
    }
}

table! {
    kinerja_ref_urjab (id) {
        id -> Integer,
        rskp_kegiatan_pk -> Nullable<Integer>,
        rjabatan_fk -> Nullable<Integer>,
        s_kd_jabdetail -> Nullable<Varchar>,
        nama_sasaran -> Nullable<Varchar>,
        target_output -> Nullable<Varchar>,
        target_kualitas -> Nullable<Varchar>,
        target_waktu -> Nullable<Integer>,
        target_biaya -> Nullable<Varchar>,
        jenis_sasaran -> Nullable<Varchar>,
    }
}

table! {
    t_backlog_project (id) {
        id -> Integer,
        judul -> Varchar,
        tgl_mulai -> Date,
        tgl_selesai -> Nullable<Date>,
        deadline -> Nullable<Date>,
    }
}

table! {
    t_backlog_catatan (id) {
        id -> Integer,
        id_project -> Integer,
        catatan -> Nullable<Varchar>,
        link_lampiran -> Nullable<Varchar>,
    }
}

table! {
    t_backlog_project_member (id) {
        id -> Integer,
        id_project -> Integer,
        niplama -> Varchar,
        nip -> Varchar,
        nama -> Varchar,
        unitkerja -> Varchar,
        peran -> Varchar,
    }
}