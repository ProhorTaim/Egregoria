use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum Language {
    English = 0,
    Russian = 1,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Russian => "ru",
        }
    }
}

impl From<u8> for Language {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Russian,
            _ => Self::English,
        }
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Self::English => "English",
            Self::Russian => "Русский",
        }
    }
}

#[derive(Default)]
pub struct I18n {
    lang: Language,
    en: HashMap<String, String>,
    current: HashMap<String, String>,
}

impl I18n {
    pub fn new() -> Self {
        let en = load_lang(Language::English.code());
        let mut this = Self {
            lang: Language::English,
            en,
            current: HashMap::new(),
        };
        this.set_language(Language::English);
        this
    }

    pub fn language(&self) -> Language {
        self.lang
    }

    pub fn set_language(&mut self, lang: Language) {
        if self.lang == lang && !self.current.is_empty() {
            return;
        }
        self.lang = lang;
        self.current = load_lang(lang.code());
    }

    pub fn tr<'a>(&'a self, key: &str) -> &'a str {
        if let Some(v) = self.current.get(key) {
            return v.as_str();
        }
        if let Some(v) = self.en.get(key) {
            return v.as_str();
        }
        key
    }

    pub fn try_tr<'a>(&'a self, key: &str) -> Option<&'a str> {
        self.current
            .get(key)
            .or_else(|| self.en.get(key))
            .map(String::as_str)
    }

    pub fn tr_args(&self, key: &str, args: &[(&str, String)]) -> String {
        let mut s = self.tr(key).to_string();
        for (k, v) in args {
            s = s.replace(&format!("{{{k}}}"), v);
        }
        s
    }

    pub fn proto_label(&self, proto_type: &str, name: &str, fallback: &str) -> String {
        let key = format!("proto.{proto_type}.{name}");
        self.try_tr(&key).unwrap_or(fallback).to_string()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

fn load_lang(code: &str) -> HashMap<String, String> {
    let path = format!("assets/i18n/{code}.json");
    let Ok(data) = fs::read_to_string(&path) else {
        log::warn!("i18n: missing translation file {}", path);
        return HashMap::new();
    };
    match serde_json::from_str::<HashMap<String, String>>(&data) {
        Ok(map) => map,
        Err(e) => {
            log::warn!("i18n: failed to parse {}: {}", path, e);
            HashMap::new()
        }
    }
}
