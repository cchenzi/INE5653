use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{web, HttpResponse};
use fluent_templates::Loader;

use crate::{
    config::APP_LANGUAGE,
    db::{delete_property, insert_property, search_content},
    i18n::{LANGUAGE_IDENTIFIER_MAP, LOCALES},
    property::{InputProperty, Property},
};

pub async fn health_check() -> HttpResponse {
    let message = LOCALES.lookup(
        &LANGUAGE_IDENTIFIER_MAP
            .get(APP_LANGUAGE.as_str())
            .expect("LanguageIdentifier not found"),
        "health-check",
    );
    HttpResponse::Ok().body(message)
}

pub async fn add(
    property: web::Json<InputProperty>,
    content: web::Data<Arc<Mutex<Vec<Property>>>>,
) -> HttpResponse {
    let mut content = content.lock().unwrap();
    let response_str = match insert_property(&mut content, property.into_inner()) {
        Ok(id) => {
            let args: HashMap<String, _> = {
                let mut map = HashMap::new();
                map.insert(String::from("id"), id.to_string().into());
                map
            };
            LOCALES.lookup_with_args(
                &LANGUAGE_IDENTIFIER_MAP
                    .get(APP_LANGUAGE.as_str())
                    .expect("LanguageIdentifier not found"),
                "added-resource",
                &args,
            )
        }
        Err(_) => LOCALES.lookup(
            &LANGUAGE_IDENTIFIER_MAP
                .get(APP_LANGUAGE.as_str())
                .expect("LanguageIdentifier not found"),
            "default-error",
        ),
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
    let fluent_args: HashMap<String, _> = {
        let mut map = HashMap::new();
        map.insert(String::from("id"), id.into());
        map
    };
    let response_str = match deleted {
        true => LOCALES.lookup_with_args(
            &LANGUAGE_IDENTIFIER_MAP
                .get(APP_LANGUAGE.as_str())
                .expect("LanguageIdentifier not found"),
            "deleted-resource",
            &fluent_args,
        ),
        false => LOCALES.lookup_with_args(
            &LANGUAGE_IDENTIFIER_MAP
                .get(APP_LANGUAGE.as_str())
                .expect("LanguageIdentifier not found"),
            "not-found-resource",
            &fluent_args,
        ),
    };
    println!("{}", response_str);
    HttpResponse::Ok().body(response_str)
}

pub async fn get(id: web::Bytes, content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let id_as_str = std::str::from_utf8(&id).expect("Unable to convert id bytes to str!");
    let id = &uuid::Uuid::parse_str(id_as_str).expect("Id isn't an UUID!");
    let content = content.lock().unwrap();
    let property = search_content(&content, id);
    let message = match property {
        Ok(v) => format!("{}", v),
        Err(_) => {
            let fluent_args: HashMap<String, _> = {
                let mut map = HashMap::new();
                map.insert(String::from("id"), id_as_str.into());
                map
            };
            LOCALES.lookup_with_args(
                &LANGUAGE_IDENTIFIER_MAP
                    .get(APP_LANGUAGE.as_str())
                    .expect("LanguageIdentifier not found"),
                "not-found-resource",
                &fluent_args,
            )
        }
    };
    println!("{}", message);
    HttpResponse::Ok().body(message)
}

pub async fn list(content: web::Data<Arc<Mutex<Vec<Property>>>>) -> HttpResponse {
    let content = content.lock().unwrap();
    let list_str = match content.len() {
        0 => LOCALES.lookup(
            &LANGUAGE_IDENTIFIER_MAP
                .get(APP_LANGUAGE.as_str())
                .expect("LanguageIdentifier not found"),
            "empty-list-resource",
        ),
        _ => crate::property::display_property_list(&content),
    };
    println!("{:?}", list_str);
    HttpResponse::Ok().body(list_str)
}
