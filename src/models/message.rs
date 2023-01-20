use serde::Serialize;

use super::Template;

const WHATSAPP: &str = "whatsapp";
const TEXT: &str = "text";
const TEMPLATE: &str = "template";

#[derive(Serialize)]
pub struct Message {
    to: String,
    messaging_product: String,
    recipient_type: Option<String>,
    #[serde(rename = "type")]
    message_type: String,
    text: Option<Text>,
    template: Option<Template>,
}

impl Message {
    pub fn from_text(to: &str, message: &str) -> Self {
        let text = Text::new(message);
        Self {
            to: to.into(),
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            message_type: TEXT.into(),
            text: Some(text),
            template: None,
        }
    }

    pub fn from_template(to: &str, template: Template) -> Self {
        Self {
            messaging_product: WHATSAPP.into(),
            recipient_type: None,
            message_type: TEMPLATE.into(),
            to: to.into(),
            text: None,
            template: Some(template),
        }
    }
}

#[derive(Serialize)]
pub struct Text {
    body: String,
}

impl Text {
    pub fn new(body: &str) -> Text {
        Self {
            body: body.into(),
        }
    }
}
