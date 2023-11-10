use crate::{
    ai_functions::ai_fun_architech::{print_project_scope, print_site_urls},
    helpers::genral::ai_task_request_decoded,
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTraits,
    },
};

use super::agent_trait::{FactSheet, ProjectScope, SpacialFunctions};

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    pub attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective:
                "Gathers information about the environment and creates a solution to the problem"
                    .to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovering,
            memory: vec![],
        };
        Self { attributes }
    }
    async fn call_project_scope(&mut self, factcheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{:?}", factcheet.project_description);
        let ai_responce: ProjectScope = ai_task_request_decoded(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;
        factcheet.project_scope = Some(ai_responce.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_responce
    }

    async fn call_determine_external_ulrs(
        &mut self,
        factcheet: &mut FactSheet,
        msg_context: String,
    ) {
        let msg_context: String = format!("{:?}", factcheet.project_description);
        let ai_responce: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;
        factcheet.external_urls = Some(ai_responce);
        self.attributes.state = AgentState::UnitTesting;
    }
}
