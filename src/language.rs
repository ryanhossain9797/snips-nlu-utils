use failure::format_err;
use std::str::FromStr;

const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const SPACE: &str = " ";

macro_rules! language_enum {
    ([$($language:ident),*]) => {
        /// Enumerates all language supported
        #[allow(non_camel_case_types)]
        #[derive(Copy,Clone,Debug)]
        pub enum Language {
            $( $language, )*
        }

        impl Language {
            pub fn all() -> Vec<Language> {
                vec![
                    $( Language::$language, )*
                ]
            }
        }
    }
}

language_enum!([DE, EN, ES, FR, IT, JA, KO, PT_PT, PT_BR]);

impl FromStr for Language {
    type Err = failure::Error;
    fn from_str(it: &str) -> Result<Language, Self::Err> {
        match &*it.to_lowercase() {
            "de" => Ok(Language::DE),
            "en" => Ok(Language::EN),
            "es" => Ok(Language::ES),
            "fr" => Ok(Language::FR),
            "it" => Ok(Language::IT),
            "ja" => Ok(Language::JA),
            "ko" => Ok(Language::KO),
            "pt_pt" => Ok(Language::PT_PT),
            "pt_br" => Ok(Language::PT_BR),
            _ => Err(format_err!("Unknown language {}", it)),
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match *self {
            Language::DE => "de".to_string(),
            Language::EN => "en".to_string(),
            Language::ES => "es".to_string(),
            Language::FR => "fr".to_string(),
            Language::IT => "it".to_string(),
            Language::JA => "ja".to_string(),
            Language::KO => "ko".to_string(),
            Language::PT_PT => "pt_pt".to_string(),
            Language::PT_BR => "pt_br".to_string(),
        }
    }
}

impl Language {
    pub fn punctuation(&self) -> &'static str {
        PUNCTUATION
    }

    pub fn default_separator(&self) -> &'static str {
        SPACE
    }
}
