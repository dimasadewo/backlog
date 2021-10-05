#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate chrono_locale;
extern crate futures;
extern crate rand;
extern crate tokio_core;

use actix_web::{middleware, App, HttpServer, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Mutex;
use crate::models::User;
use actix_cors::Cors;
use diesel::MysqlConnection;
use serde::Serialize;

mod api_middleware;
mod repositories;
mod actions;
mod models;
mod schema;
mod controllers;
// mod queries;
// mod response;

#[derive(Serialize)]
pub struct JSONResult<T> {
    success: bool,
    result: T,
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct ConnectionPools {
    hrm_pool: DbPool,
}

pub struct LoginData {
    user: Option<User>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool : DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let bind = "0.0.0.0:8089";
    let login_data = web::Data::new(Mutex::new(LoginData{ user: None }));

    println!("Starting MAP Kinerja Service v0.0.1 server at: {}", &bind);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .data(ConnectionPools{
                hrm_pool: pool.clone()
            })
            .app_data(login_data.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1")
                .service(web::resource("/backlog")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_backlog))
                    .route(web::post().to(controllers::backlog::save_backlog))
                )

                .service(web::resource("/backlog/bawahan")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::post().to(controllers::backlog::get_list_bawahan))
                )

                .service(web::resource("/backlog/update")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::post().to(controllers::backlog::get_backlog_update))
                )

                .service(web::resource("/backlog/delete")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::post().to(controllers::backlog::get_backlog_delete))
                )

                .service(web::resource("/backlog/done")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::post().to(controllers::backlog::get_backlog_done))
                )

                .service(web::resource("/backlog/sasaran")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_sasaran))
                )

                .service(web::resource("/backlog/indikator")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_indikator))
                )   
                
                .service(web::resource("/backlog/urjab")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_uraian_jabatan))
                )  

                .service(web::resource("/userorg")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_userorg))
                )

                .service(web::resource("/backlog/project")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    .route(web::get().to(controllers::backlog::get_project))
                    .route(web::post().to(controllers::backlog::save_project))
                )
                
                .service(web::resource("/backlog/catatan")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    //.route(web::get().to(controllers::backlog::get_backlog))
                    .route(web::post().to(controllers::backlog::save_catatan))
                )

                .service(web::resource("/backlog/member")
                    .wrap(api_middleware::auth_middleware::AuthMiddleware)
                    //.route(web::get().to(controllers::backlog::get_backlog))
                    .route(web::post().to(controllers::backlog::save_member))
                )
            )
            .service(web::resource("/").route(web::get().to(controllers::home::index)))
    })
        .bind(&bind)?
        .run()
        .await
}