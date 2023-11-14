use crate::models::agent_basic::basic_agent::{BasicAgent, AgentState};

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bugs_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend developer".to_string(),
            state: AgentState::Discovering,
            memory: vec![]
        }
        Self {
            attributes,
            bugs_errors: None,
            bug_count: 0,
        }
    }
}
