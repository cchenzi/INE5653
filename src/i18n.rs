#![allow(unused)]

use fluent_templates::{LanguageIdentifier, Loader};
use lazy_static::lazy_static;
use std::collections::HashMap;
use unic_langid::langid;

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
        m.insert("pt_BR".to_string(), BR_PORTUGUESE);
        m.insert("en_US".to_string(), US_ENGLISH);
        m
    };
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
            map.insert(String::from("id"), "123".into());
            map
        };

        let added_en_us = LOCALES.lookup_with_args(&US_ENGLISH, "added-resource", &args);
        let added_pt_br = LOCALES.lookup_with_args(&BR_PORTUGUESE, "added-resource", &args);

        assert_eq!(
            added_en_us,
            "Property successfully added. Generated id=\u{2068}123\u{2069}!"
        );
        assert_eq!(
            added_pt_br,
            "Propriedade adicionada com sucesso. Id gerado=\u{2068}123\u{2069}!"
        );
    }
}
