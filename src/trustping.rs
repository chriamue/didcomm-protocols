//! # Trust Ping Protocol 2.0
//!
//! The trust-ping protocol defined in the DIDComm Messaging Spec. This enables the sender and recipient to engage in an exchange of trust pings.
//! <https://identity.foundation/didcomm-messaging/spec/#trust-ping-protocol-20>

use didcomm_rs::Message;
use serde_json::json;

#[derive(Default)]
pub struct TrustPingResponseBuilder {
    thid: Option<String>,
    message: Option<Message>,
}

impl TrustPingResponseBuilder {
    pub fn new() -> Self {
        TrustPingResponseBuilder {
            thid: None,
            message: None,
        }
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn thid(&mut self, thid: String) -> &mut Self {
        self.thid = Some(thid);
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        match &self.message {
            Some(message) => match message.get_didcomm_header().m_type.as_str() {
                "https://didcomm.org/trust-ping/2.0/ping" => self.build_response(),
                _ => Err("unsupported message"),
            },
            None => self.build_ping(),
        }
    }

    pub fn build_ping(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://didcomm.org/trust-ping/2.0/ping")
            .body(&json!({"response_requested": true}).to_string()))
    }

    pub fn build_response(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://didcomm.org/trust-ping/2.0/ping-response")
            .thid(
                self.thid
                    .as_ref()
                    .unwrap_or_else(|| &self.message.as_ref().unwrap().get_didcomm_header().id),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ping() {
        let response = TrustPingResponseBuilder::new().build().unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/trust-ping/2.0/ping"
        );

        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }

    #[test]
    fn test_build_response() {
        let ping = TrustPingResponseBuilder::new().build().unwrap();

        assert_eq!(
            ping.get_didcomm_header().m_type,
            "https://didcomm.org/trust-ping/2.0/ping"
        );

        let response = TrustPingResponseBuilder::new()
            .message(ping)
            .build()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/trust-ping/2.0/ping-response"
        );

        let response = TrustPingResponseBuilder::new()
            .thid("42".to_string())
            .build_response()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/trust-ping/2.0/ping-response"
        );

        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}
