use actix_web::{web, HttpResponse};

use crate::property::Property;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn add(property: web::Json<Property>) -> HttpResponse {
    println!("{:?}", property);
    HttpResponse::Ok().finish()
}
