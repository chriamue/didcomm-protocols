//! # issuecredential
//!
//! protocol for issuing credentials. This is the basis of interoperability between Issuers and Holders.
//! <https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2/README.md>
//! ![](https://github.com/hyperledger/aries-rfcs/raw/main/features/0453-issue-credential-v2/credential-issuance.png)

use base64::encode;
use didcomm_rs::{AttachmentBuilder, AttachmentDataBuilder, Message};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2/README.md#preview-credential
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CredentialPreview {
    #[serde(rename = "type")]
    pub type_: String,
    pub attributes: Vec<CredentialAttribute>,
}

// if mime-type is not null, then value is always a base64url-encoded string that represents a binary BLOB, and mime-type tells how to interpret the BLOB after base64url-decoding.

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CredentialAttribute {
    pub name: String,
    #[serde(rename = "mime-type")]
    pub mime_type: Option<String>,
    pub value: String,
}

impl CredentialAttribute {
    pub fn new(name: String, value: String) -> Self {
        CredentialAttribute {
            name,
            value,
            mime_type: None,
        }
    }
}

/// Issue Credential Response Builder
///
/// # Examples
///
/// ```
/// use serde_json::Value;
/// use didcomm_protocols::IssueCredentialResponseBuilder;
/// let credential = Value::String("Credential".to_string());
/// let response = IssueCredentialResponseBuilder::new()
///     .attachment(credential)
///     .build_issue_credential()
///     .unwrap();
/// assert_eq!(response.get_didcomm_header().m_type,
///     "https://didcomm.org/issue-credential/2.1/issue-credential");
/// ```
#[derive(Default)]
pub struct IssueCredentialResponseBuilder {
    comment: Option<String>,
    credential_preview: Option<CredentialPreview>,
    did: Option<String>,
    did_doc: Option<Value>,
    goal_code: Option<String>,
    message: Option<Message>,
    replacement_id: Option<String>,
    attachments: Vec<Value>,
}

impl IssueCredentialResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn credential_preview(&mut self, credential_preview: CredentialPreview) -> &mut Self {
        self.credential_preview = Some(credential_preview);
        self
    }

    pub fn did(&mut self, did: String) -> &mut Self {
        self.did = Some(did);
        self
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn did_doc(&mut self, did_doc: Value) -> &mut Self {
        self.did_doc = Some(did_doc);
        self
    }

    pub fn goal_code(&mut self, goal_code: String) -> &mut Self {
        self.goal_code = Some(goal_code);
        self
    }

    pub fn replacement_id(&mut self, replacement_id: String) -> &mut Self {
        self.replacement_id = Some(replacement_id);
        self
    }

    pub fn attachment(&mut self, attachment: Value) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        match &self.message {
            Some(message) => match message.get_didcomm_header().m_type.as_str() {
                "https://didcomm.org/issue-credential/2.1/offer-credential" => {
                    self.build_propose_credential()
                }
                _ => {
                    println!("{}", message.get_didcomm_header().m_type.as_str());
                    Err("unsupported message")
                }
            },
            None => Err("no message"),
        }
    }

    pub fn build_propose_credential(&mut self) -> Result<Message, &'static str> {
        let mut message = Message::new()
            .m_type("https://didcomm.org/issue-credential/2.1/propose-credential")
            .add_header_field(
                "credential_preview".to_string(),
                serde_json::to_string(&self.credential_preview.as_ref().unwrap()).unwrap(),
            );
        if let Some(comment) = self.comment.as_ref() {
            message = message.add_header_field("comment".to_string(), comment.to_string())
        }
        if let Some(goal_code) = self.goal_code.as_ref() {
            message = message.add_header_field("goal_code".to_string(), goal_code.to_string())
        }
        Ok(message)
    }

    pub fn build_offer_credential(&mut self) -> Result<Message, &'static str> {
        let mut message = Message::new()
            .m_type("https://didcomm.org/issue-credential/2.1/offer-credential")
            .add_header_field(
                "credential_preview".to_string(),
                serde_json::to_string(&self.credential_preview.as_ref().unwrap()).unwrap(),
            );

        if let Some(comment) = self.comment.as_ref() {
            message = message.add_header_field("comment".to_string(), comment.to_string())
        }
        if let Some(goal_code) = self.goal_code.as_ref() {
            message = message.add_header_field("goal_code".to_string(), goal_code.to_string())
        }
        Ok(message)
    }

    pub fn build_issue_credential(&mut self) -> Result<Message, &'static str> {
        let mut message =
            Message::new().m_type("https://didcomm.org/issue-credential/2.1/issue-credential");
        for attachment in &self.attachments {
            message.append_attachment(
                AttachmentBuilder::new(true)
                    .with_id("credential")
                    .with_media_type("application/json")
                    .with_data(AttachmentDataBuilder::new().with_encoded_payload(&encode(
                        &serde_json::to_string(attachment).unwrap(),
                    ))),
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
    fn test_build_propose_credential() {
        let credential_preview = CredentialPreview {
            type_: "".to_string(),
            attributes: vec![CredentialAttribute::new(
                "name".to_string(),
                "value".to_string(),
            )],
        };
        let response = IssueCredentialResponseBuilder::new()
            .credential_preview(credential_preview)
            .build_propose_credential()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/issue-credential/2.1/propose-credential"
        );
    }

    #[test]
    fn test_build_issue_credential() {
        let credential = Value::String("Credential".to_string());
        let response = IssueCredentialResponseBuilder::new()
            .attachment(credential)
            .build_issue_credential()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/issue-credential/2.1/issue-credential"
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
            "\"Credential\"".to_string()
        );
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}
