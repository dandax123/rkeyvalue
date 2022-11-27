mod core;
mod services;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use services::master::MasterAppData;
use std::{
    env,
    sync::{Arc, Mutex},
};

static mut STORAGE_SERVICE: Option<std::sync::Arc<Mutex<MasterAppData>>> = None;
// use clap::Parser;

/// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long, default_value_t=String::new("master"))]
//     name: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 3000)]
//     port: u16,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = core::master::db::LevelDB::new("../testdb");
    unsafe { STORAGE_SERVICE = Some(std::sync::Arc::new(Mutex::new(MasterAppData { db }))) };

    let args: Vec<String> = env::args().collect();

    let service_type = args
        .get(1)
        .expect("Usage cargo run [master|volume]")
        .to_owned();

    let service_port: u16 = args
        .get(2)
        .expect(format!("Provide port for the {}", service_type).as_str())
        .parse()
        .unwrap();

    HttpServer::new(move || {
        App::new().configure(if (service_type == "master") {
            services::master::master_service
        } else {
            services::volume::volume_service
        })
    })
    .bind(("127.0.0.1", service_port))?
    .run()
    .await
}
