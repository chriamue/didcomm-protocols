// https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2/README.md

use didcomm_rs::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2/README.md#preview-credential
#[derive(Serialize, Deserialize)]
pub struct CredentialPreview {
    #[serde(rename = "type")]
    pub type_: String,
    pub attrubutes: Vec<CredentialPreviewAttribute>,
}

// if mime-type is not null, then value is always a base64url-encoded string that represents a binary BLOB, and mime-type tells how to interpret the BLOB after base64url-decoding.

#[derive(Serialize, Deserialize)]
pub struct CredentialPreviewAttribute {
    pub name: String,
    #[serde(rename = "mime-type")]
    pub mime_type: Option<String>,
    pub value: String,
}

impl CredentialPreviewAttribute {
    pub fn new(name: String, value: String) -> Self {
        CredentialPreviewAttribute {
            name,
            value,
            mime_type: None,
        }
    }
}

#[derive(Default)]
pub struct IssueCredentialResponseBuilder {
    comment: Option<String>,
    credential_preview: Option<CredentialPreview>,
    did: Option<String>,
    did_doc: Option<Value>,
    goal_code: Option<String>,
    message: Option<Message>,
    replacement_id: Option<String>,
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

    pub fn build_offer_credential(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://didcomm.org/issue-credential/2.1/offer-credential")
            .add_header_field(
                "comment".to_string(),
                self.comment.as_ref().unwrap().to_string(),
            )
            .add_header_field(
                "goal_code".to_string(),
                self.goal_code.as_ref().unwrap().to_string(),
            )
            .add_header_field(
                "credential_preview".to_string(),
                serde_json::to_string(&self.credential_preview.as_ref().unwrap()).unwrap(),
            ))
    }

    pub fn build_propose_credential(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://didcomm.org/issue-credential/2.1/propose-credential")
            .add_header_field("goal".to_string(), "To create a relationship".to_string())
            .add_header_field("did".to_string(), self.did.clone().unwrap())
            .add_header_field(
                "did_doc~attach".to_string(),
                serde_json::to_string_pretty(&self.did_doc.clone().unwrap()).unwrap(),
            ))
    }
}
