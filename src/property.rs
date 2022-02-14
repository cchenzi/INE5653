use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputProperty {
    pub day: String,
    pub month: String,
    pub year: String,
    pub description: String,
    pub value: String,
    pub size: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub id: uuid::Uuid,
    pub value: String,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id: {}\nValue: {}", self.id, self.value)
    }
}

pub fn display_property_list(entries: &[Property]) -> String {
    entries
        .iter()
        .map(|p| format!("{}", p))
        .collect::<Vec<String>>()
        .join("\n")
}
