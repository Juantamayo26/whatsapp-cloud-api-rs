use whatsapp_cloud_api::{
    models::{Component, Message, Parameter, Template},
    WhatasppClient, WhatsappError,
};

#[tokio::test]
async fn send_text_message_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let phone_number_id =
        std::env::var("PHONE_NUMBER_ID").expect("Missing environment variable PHONE_NUMBER_ID");
    let message = Message::from_text(&to, "test message");
    let client = WhatasppClient::new(&access_token, &phone_number_id);
    let response = client.send_message(&message).await?;
    assert_eq!(response.messages.len(), 1);
    Ok(())
}

#[tokio::test]
async fn send_message_template_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let phone_number_id =
        std::env::var("PHONE_NUMBER_ID").expect("Missing environment variable PHONE_NUMBER_ID");
    let template_name = "hello_world";
    let language = "en_US";
    let template = Template::new(template_name, language);
    let message = Message::from_template(&to, template);
    let client = WhatasppClient::new(&access_token, &phone_number_id);
    let response = client.send_message(&message).await?;
    assert_eq!(response.messages.len(), 1);
    Ok(())
}

#[tokio::test]
async fn send_message_template_with_components_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let phone_number_id =
        std::env::var("PHONE_NUMBER_ID").expect("Missing environment variable PHONE_NUMBER_ID");
    let template_name = "sample_shipping_confirmation";
    let language = "en_US";
    let parameters = Vec::from([Parameter::from_text("3")]);
    let components = Vec::from([Component::with_parameters("body", parameters)]);
    let template = Template::with_components(template_name, language, components);
    let message = Message::from_template(&to, template);
    let client = WhatasppClient::new(&access_token, &phone_number_id);
    let response = client.send_message(&message).await?;
    assert_eq!(response.messages.len(), 1);
    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
