use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse};

use crate::{
    db::{delete_property, insert_property, search_content},
    property::{InputProperty, Property},
};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}

pub async fn add(
    property: web::Json<InputProperty>,
    content: web::Data<Arc<Mutex<Vec<Property>>>>,
) -> HttpResponse {
    let mut content = content.lock().unwrap();
    let response_str = match insert_property(&mut content, property.into_inner()) {
        Ok(id) => format!("Property successfully added with uuid={}", id),
        Err(e) => e,
    };
    println!("{}", response_str);
    HttpResponse::Ok().body(response_str)
}

pub async fn delete(id: web::Bytes, content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let id = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let mut content = content.lock().unwrap();
    let deleted = delete_property(
        &mut content,
        &uuid::Uuid::parse_str(id).expect("Id isn't an UUID!"),
    );
    let response_str = match deleted {
        true => format!("{} was deleted with success!", id),
        false => format!("Id {} not found!", id),
    };
    println!("{}", response_str);
    HttpResponse::Ok().body(response_str)
}

pub async fn get(id: web::Bytes, content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let id = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let id = &uuid::Uuid::parse_str(id).expect("Id isn't an UUID!");
    let content = content.lock().unwrap();
    let property = search_content(&content, id);
    let message = match property {
        Ok(v) => format!("{:?}", v),
        Err(e) => e,
    };
    println!("{}", message);
    HttpResponse::Ok().body(message)
}

pub async fn list(content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let content = content.lock().unwrap();
    let list_str = match content.len() {
        0 => "No property to show!".to_string(),
        _ => crate::property::display_property_list(&content),
    };
    println!("{:?}", list_str);
    HttpResponse::Ok().body(list_str)
}
