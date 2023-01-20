use crate::{
    models::{Message, MessageResponse},
    WhatsappError,
};

const WHATSAPP_API_URL: &str = "https://graph.facebook.com/v15.0/102083622528971/messages";

pub struct WhatasppClient {
    access_token: String,
}

impl WhatasppClient {
    pub fn new(access_token: &str) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    pub async fn send_message(&self, message: &Message) -> Result<MessageResponse, WhatsappError> {
        http_client::post(WHATSAPP_API_URL, &self.access_token, message).await
    }
}

mod http_client {
    use reqwest::StatusCode;
    use crate::{
        models::{Message, MessageResponse},
        WhatsappError,
    };

    pub async fn post(url: &str, bearer_token: &str, data: &Message) -> Result<MessageResponse, WhatsappError>
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
