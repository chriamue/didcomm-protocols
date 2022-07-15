//! # Basic Message
//!
//! The BasicMessage protocol describes a stateless, easy to support user message protocol. It has a single message type used to communicate.
//! <https://didcomm.org/basicmessage/2.0/>

use didcomm_rs::Message;
use serde_json::json;

#[derive(Default)]
pub struct BasicMessageBuilder {
    message: Option<String>,
    lang: Option<String>,
}

impl BasicMessageBuilder {
    pub fn new() -> Self {
        BasicMessageBuilder {
            message: None,
            lang: Some("en".to_string()),
        }
    }

    pub fn message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn lang(&mut self, lang: String) -> &mut Self {
        self.lang = Some(lang);
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        let mut message = Message::new()
            .m_type("https://didcomm.org/basicmessage/2.0/message")
            .add_header_field(
                "created_time".to_string(),
                format!("{}", chrono::Utc::now().timestamp()),
            )
            .body(&json!({"content": self.message.as_ref().unwrap()}).to_string());
        if self.lang.is_some() {
            message = message
                .add_header_field("lang".to_string(), self.lang.as_ref().unwrap().to_string());
        }
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_build_message() {
        let message = "Hello World".to_string();
        let response = BasicMessageBuilder::new()
            .message(message.to_string())
            .build()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/basicmessage/2.0/message"
        );
        assert_eq!(
            response.get_body().unwrap(),
            serde_json::to_string(&json!({ "content": message })).unwrap()
        );
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }

    #[test]
    fn test_message_as_json() {
        let message = "Hello World".to_string();
        let response = BasicMessageBuilder::new()
            .message(message.to_string())
            .lang("en".to_string())
            .build()
            .unwrap();

        let didcomm_message = response.as_raw_json().unwrap();

        let json_value: Value = serde_json::from_str(&didcomm_message).unwrap();

        assert_eq!(json_value.get("lang").unwrap().as_str().unwrap(), "en");
        assert_eq!(
            json_value
                .get("body")
                .unwrap()
                .get("content")
                .unwrap()
                .as_str()
                .unwrap(),
            "Hello World"
        );
    }
}
