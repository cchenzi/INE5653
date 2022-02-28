use std::{collections::HashMap, fmt};

use chrono::Utc;
use fluent_templates::Loader;
use serde::{Deserialize, Serialize};

use crate::{
    config::APP_LANGUAGE,
    i18n::{LANGUAGE_IDENTIFIER_MAP, LOCALES},
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
        let fluent_args: HashMap<String, _> = {
            let mut map = HashMap::new();
            map.insert(String::from("id"), self.id.to_string().into());
            map.insert(
                String::from("build_date"),
                self.build_date.to_string().into(),
            );
            map.insert(
                String::from("created_at"),
                self.created_at.to_string().into(),
            );
            map.insert(
                String::from("description"),
                self.description.to_owned().into(),
            );
            map.insert(String::from("value"), self.value.to_owned().into());
            map.insert(String::from("size"), self.size.to_owned().into());
            map.insert(String::from("country"), self.country.to_owned().into());
            map.insert(String::from("currency"), self.currency.to_owned().into());
            map
        };
        write!(
            f,
            "{}",
            LOCALES.lookup_with_args(
                &LANGUAGE_IDENTIFIER_MAP
                    .get(APP_LANGUAGE.as_str())
                    .expect("LanguageIdentifier not found"),
                "display-resource",
                &fluent_args,
            )
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
