use crate::models::{
    agent_basic::basic_agent::BasicAgent,
    agents::agent_trait::{FactSheet, SpacialFunctions},
};

#[derive(Debug)]
pub struct ManagingAgent {
    pub attributes: BasicAgent,
    pub factsheets: FactSheet,
    pub agent: Vec<Box<dyn SpacialFunctions>>,
}
