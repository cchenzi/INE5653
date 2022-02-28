use std::fmt;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum AllowedCountries {
    BRA,
    UK,
    USA,
}

impl AllowedCountries {
    pub fn currency(&self) -> &'static str {
        match self {
            AllowedCountries::BRA => "Real",
            AllowedCountries::UK => "Pound Sterling",
            AllowedCountries::USA => "Dollar",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AllowedCountries::BRA => "Brasil",
            AllowedCountries::UK => "United Kingdon",
            AllowedCountries::USA => "United States of America",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InputProperty {
    pub day: usize,
    pub month: usize,
    pub year: usize,
    pub country: AllowedCountries,
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
    pub country: String,
    pub currency: String,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}\nBuild date: {}\nCreated at: {}\nDescription: {}\nValue: {}\nSize: {}\nCountry: {} \nCurrency: {}",
            self.id, self.build_date, self.created_at, self.description, self.value, self.size, self.country, self.currency
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
