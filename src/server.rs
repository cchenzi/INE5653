use std::sync::Mutex;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::db::init_database;
use crate::routes::{add, delete, get, health_check, list};

pub fn run() -> Result<Server, std::io::Error> {
    let data = web::Data::new(std::sync::Arc::new(Mutex::new(init_database())));
    println!("oi");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/add", web::post().to(add))
            .route("/delete", web::post().to(delete))
            .route("/get", web::post().to(get))
            .route("/list", web::get().to(list))
    })
    .bind("127.0.0.1:6666")?
    .run();

    Ok(server)
}
