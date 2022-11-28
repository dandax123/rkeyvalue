use actix_web::{get, web, HttpResponse, HttpServer, Responder};

use crate::core::volume::volume_server::VolumeServer;

async fn get_value(path: web::Path<String>) -> impl Responder {
    let key = path.into_inner();
    let y = VolumeServer::new("/tmp/volume1");
    let hkey = format!("{:x}", md5::compute(key.as_bytes()));
    let value = y.get(hkey.as_str());
    HttpResponse::Ok().body(format!("Hello Volume!: Found: {}", value))
    // "cool"
}
async fn put_value(path: web::Path<String>) -> impl Responder {
    let y = VolumeServer::new("/tmp/volume1");
    let key = path.into_inner();
    let hkey = format!("{:x}", md5::compute(key.as_bytes()));
    // println!("{}", hkey);
    y.put(hkey.as_str(), String::from("cool"));
    HttpResponse::Ok().body("Hello Volume!")
    // "cool"
}

async fn del_value(path: web::Path<String>) -> impl Responder {
    let y = VolumeServer::new("/tmp/volume1");
    let key = path.into_inner();
    let hkey = format!("{:x}", md5::compute(key.as_bytes()));
    // println!("{}", hkey);
    y.remove(hkey.as_str());
    HttpResponse::Ok().body("File Removed ")
    // "cool"
}

pub(crate) fn volume_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{key}")
            .route(web::get().to(get_value))
            .route(web::post().to(put_value))
            .route(web::put().to(put_value))
            .route(web::delete().to(del_value)),
    );
}
