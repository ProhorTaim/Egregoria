use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Language {
    English = 0,
    Russian = 1,
}

// Custom serialize/deserialize using string codes
impl serde::ser::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.code())
    }
}

impl<'de> serde::de::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let code = <&str>::deserialize(deserializer)?;
        Ok(match code {
            "ru" | "Русский" => Language::Russian,
            "en" | "English" => Language::English,
            _ => Language::English,
        })
    }
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
        // Always update language, even if it's the same as current
        // This ensures translations are reloaded
        self.lang = lang;
        self.current = load_lang(lang.code());
    }

    // Return an owned String to avoid borrowing values tied to `self`.
    // This makes it safe to pass translations into GUI APIs that require
    // `'static`-owned values or to move them into closures.
    pub fn tr(&self, key: &str) -> String {
        if let Some(v) = self.current.get(key) {
            return v.clone();
        }
        if let Some(v) = self.en.get(key) {
            return v.clone();
        }
        key.to_string()
    }

    pub fn try_tr(&self, key: &str) -> Option<String> {
        self.current
            .get(key)
            .or_else(|| self.en.get(key))
            .map(|s| s.clone())
    }

    pub fn tr_args(&self, key: &str, args: &[(&str, String)]) -> String {
        let mut s = self.tr(key);
        for (k, v) in args {
            s = s.replace(&format!("{{{k}}}"), v);
        }
        s
    }

    pub fn proto_label(&self, proto_type: &str, name: &str, fallback: &str) -> String {
        let key = format!("proto.{proto_type}.{name}");
        self.try_tr(&key).unwrap_or_else(|| fallback.to_string())
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

fn load_lang(code: &str) -> HashMap<String, String> {
    log::info!("i18n: attempting to load language '{}'", code);
    let path = format!("assets/i18n/{code}.json");
    let Ok(data) = fs::read_to_string(&path) else {
        log::warn!("i18n: missing translation file {}", path);
        return HashMap::new();
    };
    match serde_json::from_str::<HashMap<String, String>>(&data) {
        Ok(map) => {
            log::info!("Loaded i18n language {} with {} entries", code, map.len());
            map
        }
        Err(e) => {
            log::warn!("i18n: failed to parse {}: {}", path, e);
            HashMap::new()
        }
    }
}
