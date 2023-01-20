use serde::Serialize;

#[derive(Serialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub parameters: Option<Vec<Parameter>>,
}

impl Component {
    pub fn with_parameters(component_type: &str, parameters: Vec<Parameter>) -> Self {
        Self {
            component_type: component_type.into(),
            parameters: Some(parameters),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub text: Option<String>,
}

impl Parameter {
    pub fn from_text(text: &str) -> Self {
        Self {
            parameter_type: "text".into(),
            text: Some(text.into()),
        }
    }
}
