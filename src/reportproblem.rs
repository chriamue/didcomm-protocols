//! # Report Problem Protocol 1.0
//!
//! Describes how to report errors and warnings in a powerful, interoperable way. All implementations of SSI agent or hub technology SHOULD implement this RFC.
//! <https://github.com/hyperledger/aries-rfcs/blob/main/features/0035-report-problem/README.md>

use didcomm_rs::Message;
use serde_json::Value;

#[derive(Default)]
pub struct ReportProblemResponseBuilder {
    message: Option<Message>,
    description: Option<String>,
    problem_items: Vec<Value>,
}

impl ReportProblemResponseBuilder {
    pub fn new() -> Self {
        ReportProblemResponseBuilder {
            message: None,
            description: None,
            problem_items: Vec::new(),
        }
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn problem_item(&mut self, problem_item: Value) -> &mut Self {
        self.problem_items.push(problem_item);
        self
    }

    pub fn build(&mut self) -> Result<Message, &'static str> {
        let mut message =
            Message::new().m_type("https://didcomm.org/report-problem/1.0/problem-report");
        if let Some(m) = self.message.as_ref() {
            message = message.thid(&m.get_didcomm_header().id)
        }
        Ok(message)
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
            .build()
            .unwrap();
        assert_eq!(
            report.get_didcomm_header().m_type,
            "https://didcomm.org/report-problem/1.0/problem-report"
        );
        assert_eq!(report.get_didcomm_header().thid.clone().unwrap(), thid);
    }
}
