use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("MAP Rust Kinerja Service v1.0.0 based on Actix Web Framework, built on Rust.")
}