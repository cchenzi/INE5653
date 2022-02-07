use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
};

use crate::property::{InputProperty, Property};

const DB_FILE_NAME: &str = "./data/db.json";

pub fn get_content() -> Vec<Property> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(DB_FILE_NAME)
        .expect("Unable to open the db file!");
    let buf_reader = BufReader::new(file);
    let content: Vec<Property> =
        serde_json::from_reader(buf_reader).expect("Unable to reade file!");
    content
}

pub fn save_content(content: &[Property]) {
    let file = File::create(DB_FILE_NAME).expect("Unable to open the db file!");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, content).expect("Unable to reade file!")
}

pub fn search_content(id: &uuid::Uuid) -> Result<Property, String> {
    let content = get_content();
    for property in content {
        if property.id == *id {
            return Ok(property);
        }
    }
    Err("Property not found!".to_string())
}

pub fn delete_property(id: &uuid::Uuid) {
    let content = get_content();
    let filtered_content: Vec<Property> = content.into_iter().filter(|p| p.id != *id).collect();
    save_content(&filtered_content)
}

pub fn insert_property(input: InputProperty) {
    println!("oi");
    let id = uuid::Uuid::new_v4();
    let property = Property {
        id,
        value: input.value,
    };
    let mut content: Vec<Property> = get_content();
    content.push(property);
    save_content(&content);
}
