//! # Report Problem Protocol 2.0
//!
//! Describes how to report errors and warnings in a powerful, interoperable way. All implementations of SSI agent or hub technology SHOULD implement this RFC.
//! <https://identity.foundation/didcomm-messaging/spec/#problem-reports>

use didcomm_rs::Message;
use serde_json::{json, Value};

#[derive(Default)]
pub struct ReportProblemResponseBuilder {
    message: Option<Message>,
    code: Option<String>,
    comment: Option<String>,
    args: Vec<String>,
    escalate_to: Option<String>,
    ack: Vec<String>,
}

impl ReportProblemResponseBuilder {
    pub fn new() -> Self {
        ReportProblemResponseBuilder {
            message: None,
            code: None,
            comment: None,
            args: Vec::new(),
            escalate_to: None,
            ack: Vec::new(),
        }
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn code(&mut self, code: String) -> &mut Self {
        self.code = Some(code);
        self
    }

    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn escalate_to(&mut self, escalate_to: String) -> &mut Self {
        self.escalate_to = Some(escalate_to);
        self
    }

    pub fn ack(&mut self, ack: Vec<String>) -> &mut Self {
        self.ack = ack;
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        let mut message = Message::new()
            .m_type("https://didcomm.org/report-problem/2.0/problem-report")
            .body(&serde_json::to_string(&self.build_body()).unwrap());
        if let Some(m) = self.message.as_ref() {
            message = message.thid(&m.get_didcomm_header().id)
        }
        if !self.ack.is_empty() {
            message = message
                .add_header_field("ack".to_string(), serde_json::to_string(&self.ack).unwrap())
        }
        Ok(message)
    }

    pub fn build_body(&mut self) -> Value {
        let mut body: Value = json!({
            "code": self.code.as_ref().unwrap()
        });
        if self.comment.is_some() {
            body["comment"] = json!(self.comment.as_ref().unwrap());
        }
        if !self.args.is_empty() {
            body["args"] = json!(self.args);
        }
        if self.escalate_to.is_some() {
            body["escalate_to"] = json!(self.escalate_to.as_ref().unwrap());
        }
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_message() {
        let message = Message::new();
        let thid = message.get_didcomm_header().id.to_string();
        let report = ReportProblemResponseBuilder::new()
            .message(message)
            .code("a".to_string())
            .build()
            .unwrap();
        assert_eq!(
            report.get_didcomm_header().m_type,
            "https://didcomm.org/report-problem/2.0/problem-report"
        );
        assert_eq!(report.get_didcomm_header().thid.clone().unwrap(), thid);
    }

    #[test]
    fn test_report_problem_as_json() {
        let message = Message::new();
        let thid = message.get_didcomm_header().id.to_string();
        let response = ReportProblemResponseBuilder::new()
            .message(message)
            .ack(vec![thid.to_string()])
            .code("c".to_string())
            .comment("some comment".to_string())
            .escalate_to("mailto:admin@foo.org".to_string())
            .build()
            .unwrap();

        let didcomm_message = response.as_raw_json().unwrap();

        let json_value: Value = serde_json::from_str(&didcomm_message).unwrap();
        assert_eq!(
            json_value["ack"].as_str().unwrap(),
            serde_json::to_string(&json!(vec!(thid))).unwrap()
        );
        assert_eq!(json_value["body"]["code"].as_str().unwrap(), "c");
        assert_eq!(
            json_value["body"]["comment"].as_str().unwrap(),
            "some comment"
        );
        assert_eq!(
            json_value["body"]["escalate_to"].as_str().unwrap(),
            "mailto:admin@foo.org"
        );
    }
}
