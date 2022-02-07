use actix_web::{web, HttpResponse};

use crate::{
    db::{get_content, insert_property, search_content},
    property::InputProperty,
};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn add(property: web::Json<InputProperty>) -> HttpResponse {
    insert_property(property.into_inner());
    HttpResponse::Ok().finish()
}

pub async fn get(id: web::Bytes) -> HttpResponse {
    let id = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let property = search_content(&uuid::Uuid::parse_str(id).expect("Id isn't an UUID!"));
    match property {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
    HttpResponse::Ok().finish()
}

pub async fn list() -> HttpResponse {
    let content = get_content();
    println!("{:?}", content);
    HttpResponse::Ok().finish()
}
