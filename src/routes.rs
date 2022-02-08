use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse};

use crate::{
    db::{delete_property, insert_property, search_content},
    property::{InputProperty, Property},
};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn add(
    property: web::Json<InputProperty>,
    content: web::Data<Arc<Mutex<Vec<Property>>>>,
) -> HttpResponse {
    let mut content = content.lock().unwrap();
    insert_property(&mut content, property.into_inner());
    HttpResponse::Ok().finish()
}

pub async fn delete(id: web::Bytes, content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let id = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let mut content = content.lock().unwrap();
    delete_property(
        &mut content,
        &uuid::Uuid::parse_str(id).expect("Id isn't an UUID!"),
    );
    HttpResponse::Ok().finish()
}

pub async fn get(id: web::Bytes, content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let id = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let content = content.lock().unwrap();
    let property = search_content(
        &content,
        &uuid::Uuid::parse_str(id).expect("Id isn't an UUID!"),
    );
    match property {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
    HttpResponse::Ok().finish()
}

pub async fn list(content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let content = content.lock().unwrap();
    println!("{:?}", content);
    HttpResponse::Ok().finish()
}
