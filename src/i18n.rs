#![allow(unused)]

use chrono::Utc;
use fluent_templates::{
    fluent_bundle::{
        types::{FluentNumber, FluentNumberCurrencyDisplayStyle, FluentNumberOptions},
        FluentValue,
    },
    LanguageIdentifier, Loader,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use unic_langid::langid;

use crate::config::APP_LANGUAGE;

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

pub const US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub const BR_PORTUGUESE: LanguageIdentifier = langid!("pt-BR");

lazy_static! {
    pub static ref LANGUAGE_IDENTIFIER_MAP: HashMap<String, LanguageIdentifier> = {
        let mut m = HashMap::new();
        m.insert("pt-BR".to_string(), BR_PORTUGUESE);
        m.insert("en-US".to_string(), US_ENGLISH);
        m
    };
}

pub fn format_currency_value(currency: &str, value: f64) -> String {
    match APP_LANGUAGE.as_str() {
        "pt-BR" => format!("{} {}", currency, value.to_string().replace(".", ",")),
        "en-US" => format!("{} {}", currency, value),
        _ => "ERROR".to_string(),
    }
}

pub fn format_size_value(size: f64) -> String {
    match APP_LANGUAGE.as_str() {
        "pt-BR" => format!("{}m²", size),
        // m² to feet²
        "en-US" => format!("{}ft²", size * 10.76),
        _ => "ERROR".to_string(),
    }
}

pub fn format_naive_date(date: &chrono::NaiveDate) -> String {
    match APP_LANGUAGE.as_str() {
        "pt-BR" => date.format("%d/%m/%Y").to_string(),
        "en-US" => date.format("%m/%d/%Y").to_string(),
        _ => "ERROR".to_string(),
    }
}

pub fn format_date_time(datetime: &chrono::DateTime<Utc>) -> String {
    let locale = match APP_LANGUAGE.as_str() {
        "pt-BR" => chrono::Locale::pt_BR,
        "en-US" => chrono::Locale::en_US,
        _ => return "ERROR".to_string(),
    };
    datetime
        .format_localized("%A, %e %B %Y, %T", locale)
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_messages_without_args() {
        let hello_en_us = LOCALES.lookup(&US_ENGLISH, "hello-world");
        let hello_pt_br = LOCALES.lookup(&BR_PORTUGUESE, "hello-world");

        assert_eq!(hello_en_us, "Hello World!");
        assert_eq!(hello_pt_br, "eae!");
    }

    #[test]
    fn test_messages_with_args() {
        let args: HashMap<String, _> = {
            let mut map = HashMap::new();
            map.insert(String::from("code"), "INE5653".into());
            map.insert(String::from("name"), "i18n".into());
            map
        };

        let args_en_us = LOCALES.lookup_with_args(&US_ENGLISH, "test-args", &args);
        let args_pt_br = LOCALES.lookup_with_args(&BR_PORTUGUESE, "test-args", &args);

        assert_eq!(
            args_en_us,
            "Code: \u{2068}INE5653\u{2069}; Name: \u{2068}i18n\u{2069}"
        );
        assert_eq!(
            args_pt_br,
            "Código: \u{2068}INE5653\u{2069}; Nome: \u{2068}i18n\u{2069}"
        );
    }
}
