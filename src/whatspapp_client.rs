use crate::{
    models::{Message, MessageResponse},
    WhatsappError,
};

fn get_whatsapp_api_url(phone_number_id: &str) -> String {
    format!("https://graph.facebook.com/v15.0/{}/messages", phone_number_id)
}

#[derive(Clone)]
pub struct WhatasppClient {
    access_token: String,
    phone_number_id: String
}

impl WhatasppClient {
    pub fn new(access_token: &str, phone_number_id: &str) -> Self {
        Self {
            access_token: access_token.into(),
            phone_number_id: phone_number_id.into()
        }
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(get_whatsapp_api_url(&self.phone_number_id), &self.access_token, message).await
    }
}

mod http_client {
    use reqwest::StatusCode;
    use crate::{
        models::{Message, MessageResponse},
        WhatsappError,
    };

    pub async fn post(url: String, bearer_token: &str, data: &Message) -> Result<MessageResponse, WhatsappError>
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&bearer_token)
            .json(&data)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let json = resp.json::<MessageResponse>().await?;
                Ok(json)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
}
