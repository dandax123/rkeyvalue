use actix_web::{get, web, HttpResponse, HttpServer, Responder};

async fn my_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Master!")
    // "cool"
}

pub(crate) fn master_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/").route(web::get().to(my_hello)), // .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}