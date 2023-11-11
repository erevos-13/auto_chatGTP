use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    ai_functions::ai_fun_architech::{print_project_scope, print_site_urls},
    helpers::{
        command_line::PrintCommand,
        genral::{ai_task_request_decoded, check_status_code},
    },
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

#[async_trait]
impl SpacialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agents(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovering => {
                    let project_project = self.call_project_scope(factsheet).await;
                    if project_project.is_external_urls_required {
                        self.call_determine_external_ulrs(
                            factsheet,
                            factsheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    } else {
                        self.attributes.state = AgentState::Finished;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];
                    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
                    let urls = factsheet.external_urls.as_ref().expect("No urls found");

                    for url in urls {
                        let endpoint_str = format!("Testing url endpoint");
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str(),
                        );
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => {
                                println!("Error checking {} : {}", url, e)
                            }
                        };
                    }
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        factsheet.external_urls = Some(new_urls);
                    }
                    self.attributes.state = AgentState::Finished;
                }
                _ => self.attributes.state = AgentState::Finished,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();
        let mut factsheet = FactSheet {
            project_description: "Build a full stach web site user login lougout".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_end_point_schema: None,
        };
        agent
            .execute(&mut factsheet)
            .await
            .expect("error man mlkia");
        dbg!(factsheet);
    }
}
