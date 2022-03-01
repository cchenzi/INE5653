use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use chrono::{TimeZone, Utc};

use crate::property::{InputProperty, Property};

const DB_FILE_NAME: &str = "./data/db.json";

pub fn init_database() -> Vec<Property> {
    println!("Starting database...");

    let file = File::open(DB_FILE_NAME);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            println!(
                "Unable to read file! Error = {}. \nCreating empty file...",
                e
            );
            save_content(&[]);
            return Vec::new();
        }
    };
    let buf_reader = BufReader::new(file);
    let content: Vec<Property> =
        serde_json::from_reader(buf_reader).expect("Unable to reade file!");
    content
}

pub fn save_content(content: &[Property]) {
    println!("Saving content to file...");
    let file = File::create(DB_FILE_NAME).expect("Unable to open the db file!");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, content).expect("Unable to read file!")
}

pub fn search_content(content: &[Property], id: &uuid::Uuid) -> Result<Property, String> {
    for property in content {
        if property.id == *id {
            return Ok(property.clone());
        }
    }
    Err("Property not found!".to_string())
}

pub fn delete_property(content: &mut Vec<Property>, id: &uuid::Uuid) -> bool {
    let size = content.len();
    content.retain(|p| p.id != *id);
    save_content(content);
    size - 1 == content.len()
}

pub fn insert_property(
    content: &mut Vec<Property>,
    input: InputProperty,
) -> Result<uuid::Uuid, String> {
    let size = content.len();
    let id = uuid::Uuid::new_v4();
    let property = Property {
        build_date: Utc
            .ymd(input.year as i32, input.month as u32, input.day as u32)
            .naive_utc(),
        created_at: Utc::now(),
        description: input.description,
        id,
        value: Property::parse_value(&input.value),
        size: Property::harmonize_size(&input.country, input.size as f64),
        country: input.country.as_str().to_string(),
        currency: input.country.currency().to_string(),
    };
    content.push(property);
    save_content(content);
    match size + 1 == content.len() {
        true => Ok(id),
        false => Err("Error".to_string()),
    }
}
