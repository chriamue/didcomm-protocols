//! # issuecredential
//!
//! A protocol supporting a general purpose verifiable presentation exchange regardless of the specifics of the underlying verifiable presentation
//! request and verifiable presentation format.
//! <https://github.com/hyperledger/aries-rfcs/blob/main/features/0454-present-proof-v2/README.md>
//! ![](https://github.com/hyperledger/aries-rfcs/raw/main/features/0454-present-proof-v2/presentation-choreography.png)

use base64::encode;
use didcomm_rs::{AttachmentBuilder, AttachmentDataBuilder, Message};
use serde_json::Value;

/// Present Proof Response Builder
///
#[derive(Default)]
pub struct PresentProofResponseBuilder {
    comment: Option<String>,
    goal_code: Option<String>,
    message: Option<Message>,
    attachments: Vec<Value>,
}

impl PresentProofResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn goal_code(&mut self, goal_code: String) -> &mut Self {
        self.goal_code = Some(goal_code);
        self
    }

    pub fn attachment(&mut self, attachment: Value) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    pub fn build_presentation(&mut self) -> Result<Message, &'static str> {
        let mut message =
            Message::new().m_type("https://didcomm.org/present-proof/2.1/presentation");
        for attachment in &self.attachments {
            message.append_attachment(
                AttachmentBuilder::new(true)
                    .with_id("presentation")
                    .with_media_type("application/json")
                    .with_data(
                        AttachmentDataBuilder::new()
                            .with_link("")
                            .with_encoded_payload(&encode(
                                &serde_json::to_string(attachment).unwrap(),
                            )),
                    ),
            );
        }

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::decode;
    use std::str::from_utf8;

    #[test]
    fn test_build_presentation() {
        let presentation = Value::String("Presentation".to_string());
        let response = PresentProofResponseBuilder::new()
            .attachment(presentation)
            .build_presentation()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/present-proof/2.1/presentation"
        );
        let response_json = response.as_raw_json().unwrap();
        let response: Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(
            from_utf8(
                &decode(
                    response["attachments"][0]["data"]["base64"]
                        .as_str()
                        .unwrap()
                )
                .unwrap()
            )
            .unwrap(),
            "\"Presentation\"".to_string()
        );
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}
