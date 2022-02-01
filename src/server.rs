use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::routes::{add, health_check};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/add", web::post().to(add))
    })
    .bind("127.0.0.1:6666")?
    .run();

    Ok(server)
}
