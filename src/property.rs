use std::fmt;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputProperty {
    pub day: usize,
    pub month: usize,
    pub year: usize,
    pub description: String,
    pub value: String,
    pub size: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub build_date: chrono::NaiveDate,
    pub created_at: chrono::DateTime<Utc>,
    pub description: String,
    pub id: uuid::Uuid,
    pub value: String,
    pub size: usize,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}\nBuild date: {}\nCreated at: {}\nDescription: {}\nValue: {}\nSize: {}",
            self.id, self.build_date, self.created_at, self.description, self.value, self.size
        )
    }
}

pub fn display_property_list(entries: &[Property]) -> String {
    entries
        .iter()
        .map(|p| format!("{}", p))
        .collect::<Vec<String>>()
        .join("\n========================================\n")
}
