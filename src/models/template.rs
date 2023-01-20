use serde::Serialize;

use super::component::Component;

#[derive(Serialize)]
pub struct Template {
    name: String,
    language: Language,
    components: Option<Vec<Component>>,
}

impl Template {
    pub fn new(name: &str, language: &str) -> Self {
        let language = Language::new(language);
        Self {
            name: name.into(),
            language,
            components: None,
        }
    }

    pub fn with_components(name: &str, language: &str, components: Vec<Component>) -> Self {
        let language = Language::new(language);
        Self {
            name: name.into(),
            language,
            components: Some(components),
        }
    }
}

#[derive(Serialize)]
pub struct Language {
    code: String,
}

impl Language {
    pub fn new(code: &str) -> Language {
        Self {
            code: code.into(),
        }
    }
}
