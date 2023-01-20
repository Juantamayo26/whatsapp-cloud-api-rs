use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub contacts: Vec<ContactResponse>,
    pub messages: Vec<CreatedMessage>,
    pub messaging_product: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatedMessage {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ContactResponse {
    pub input: String,
    pub wa_id: String,
}
