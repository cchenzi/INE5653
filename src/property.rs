use std::{collections::HashMap, fmt};

use chrono::Utc;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};

use crate::{
    config::APP_LANGUAGE,
    i18n::{
        format_currency_value, format_date_time, format_naive_date, format_size_value,
        LANGUAGE_IDENTIFIER_MAP, LOCALES,
    },
};

#[derive(Deserialize, Serialize, Debug)]
pub enum AllowedCountries {
    BRA,
    UK,
    USA,
}

impl AllowedCountries {
    pub fn currency(&self) -> &'static str {
        match self {
            AllowedCountries::BRA => "R$",
            AllowedCountries::UK => "£",
            AllowedCountries::USA => "$",
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
    pub size: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub build_date: chrono::NaiveDate,
    pub created_at: chrono::DateTime<Utc>,
    pub description: String,
    pub id: uuid::Uuid,
    pub value: f64,
    pub size: f64,
    pub country: String,
    pub currency: String,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_value = format_currency_value(&self.currency, self.value);
        let formatted_size = format_size_value(self.size);

        let build_date = format_naive_date(&self.build_date);
        let created_at = format_date_time(&self.created_at);

        let fluent_args: HashMap<String, _> = {
            let mut map = HashMap::new();
            map.insert(String::from("id"), self.id.to_string().into());
            map.insert(String::from("build_date"), build_date.into());
            map.insert(String::from("created_at"), created_at.into());
            map.insert(
                String::from("description"),
                self.description.to_owned().into(),
            );
            map.insert(String::from("value"), formatted_value.into());
            map.insert(String::from("size"), formatted_size.into());
            map.insert(String::from("country"), self.country.to_owned().into());
            map
        };
        write!(
            f,
            "{}",
            LOCALES.lookup_with_args(
                LANGUAGE_IDENTIFIER_MAP
                    .get(APP_LANGUAGE.as_str())
                    .expect("LanguageIdentifier not found"),
                "display-resource",
                &fluent_args,
            )
        )
    }
}

impl Property {
    pub fn parse_value(value: &str) -> f64 {
        value.trim().replace(",", ".").parse::<f64>().unwrap()
    }
    pub fn harmonize_size(country: &AllowedCountries, size: f64) -> f64 {
        match country {
            // feet² to m²
            AllowedCountries::USA => size * 0.92,
            // rest of the world is fine, lol
            _ => size,
        }
    }
}

pub fn display_property_list(entries: &[Property]) -> String {
    entries
        .iter()
        .map(|p| format!("{}", p))
        .collect::<Vec<String>>()
        .join("\n========================================\n")
}
