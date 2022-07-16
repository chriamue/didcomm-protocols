//! # Invitation
//!
//! The out-of-band protocol consists in a single message that is sent by the sender.
//! <https://identity.foundation/didcomm-messaging/spec/#invitation>

use crate::service::Service;
use didcomm_rs::{AttachmentBuilder, AttachmentDataBuilder, Message};
use serde_json::{json, Value};

pub enum GoalCode {
    StreamlinedVC,
    StreamlinedVP,
    Other(String),
}

#[derive(Default)]
pub struct InvitationBuilder {
    goal_code: Option<String>,
    goal: Option<String>,
    attachments: Option<Vec<Message>>,
    services: Option<Vec<Service>>,
}

impl InvitationBuilder {
    pub fn new() -> Self {
        InvitationBuilder {
            goal: None,
            goal_code: None,
            attachments: None,
            services: None,
        }
    }

    pub fn goal(&mut self, goal: String) -> &mut Self {
        self.goal = Some(goal);
        self
    }

    pub fn goal_code(&mut self, goal_code: GoalCode) -> &mut Self {
        self.goal_code = match goal_code {
            GoalCode::StreamlinedVC => Some("streamlined-vc".to_string()),
            GoalCode::StreamlinedVP => Some("streamlined-vp".to_string()),
            GoalCode::Other(goal_code) => Some(goal_code),
        };
        self
    }

    pub fn attachments(&mut self, attachments: Vec<Message>) -> &mut Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn services(&mut self, services: Vec<Service>) -> &mut Self {
        self.services = Some(services);
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        let mut message = Message::new()
            .m_type("https://didcomm.org/out-of-band/2.0/invitation")
            .body(&serde_json::to_string(&self.build_body()).unwrap());

        if self.attachments.is_some() {
            for attachment in self.attachments.as_ref().unwrap() {
                let id = attachment.get_didcomm_header().id.clone();
                let attachment_json = attachment.clone().as_raw_json().unwrap();
                message.append_attachment(
                    AttachmentBuilder::new(true).with_id(&id).with_data(
                        AttachmentDataBuilder::new()
                            .with_link("")
                            .with_json(&attachment_json),
                    ),
                );
            }
        }
        if self.services.is_some() {
            message = message.add_header_field(
                "services".to_string(),
                serde_json::to_string(&self.services).unwrap(),
            );
        }

        Ok(message)
    }

    pub fn build_body(&mut self) -> Value {
        let mut body: Value = json!({
            "goal_code": self.goal_code.as_ref().unwrap(), "accept": [
            "didcomm/v2"
          ]
        });
        if self.goal.is_some() {
            body["goal"] = json!(self.goal.as_ref().unwrap());
        }
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_invitation() {
        let dummy = Message::new();

        let response = InvitationBuilder::new()
            .goal("goal".to_string())
            .goal_code(GoalCode::Other("goal_code".to_string()))
            .attachments(vec![dummy])
            .build()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://didcomm.org/out-of-band/2.0/invitation"
        );
        assert!(response.get_attachments().next().is_some());

        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }

    #[test]
    fn test_invitation_as_json() {
        let response = InvitationBuilder::new()
            .goal("goal".to_string())
            .goal_code(GoalCode::StreamlinedVC)
            .build()
            .unwrap();

        let didcomm_message = response.as_raw_json().unwrap();

        let json_value: Value = serde_json::from_str(&didcomm_message).unwrap();
        assert_eq!(
            json_value["body"]["goal_code"].as_str().unwrap(),
            "streamlined-vc"
        );
    }
}
