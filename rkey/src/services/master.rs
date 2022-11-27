use crate::{
    core::master::{self, db::LevelDB},
    STORAGE_SERVICE,
};
use actix_web::{get, web, HttpResponse, HttpServer, Responder};

use std::sync::{Arc, Mutex};
pub(crate) struct MasterAppData {
    pub db: LevelDB,
}

async fn del_val(path: web::Path<String>) -> impl Responder {
    HttpResponse::NotFound().body("Can't find")
}
async fn put_val(path: web::Path<String>) -> impl Responder {
    HttpResponse::NotFound().body("Can't find")
}
async fn get_val(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    {
        let y = unsafe {
            STORAGE_SERVICE
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .db
                .get(key.as_str())
        };
        match y {
            Some(_) => HttpResponse::Ok().body(format!("Hello Master! {:?}", y)),
            None => HttpResponse::NotFound().body("Can't find"),
        }
    }

    // const db = _conn.D

    // "cool"
}

pub(crate) fn master_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{key}")
            .route(web::get().to(get_val))
            .route(web::delete().to(del_val))
            .route(web::put().to(put_val))
            .route(web::post().to(put_val)),
    );
}
