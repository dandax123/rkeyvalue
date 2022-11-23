use crate::core::master;
use actix_web::{get, web, HttpResponse, HttpServer, Responder};
use rusty_leveldb::DB;
use std::sync::{Arc, Mutex};
pub(crate) struct MasterAppData {
    pub db: DB,
}

async fn get_val(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    // const db = _conn.D
    HttpResponse::Ok().body(format!("Hello Master! {}", key))
    // "cool"
}

pub(crate) fn master_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{key}").route(web::get().to(get_val)));
}
