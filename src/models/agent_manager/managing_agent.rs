use crate::{
    ai_functions::ai_fun_manager::convert_user_input_to_goal,
    helpers::genral::{ai_task_request, ai_task_request_decoded},
    models::{
        agent_basic::basic_agent::{AgentState, BasicAgent},
        agents::{
            agent_architect::AgentSolutionArchitect,
            agent_trait::{FactSheet, ProjectScope, SpacialFunctions},
        },
    },
};

#[derive(Debug)]
pub struct ManagingAgent {
    pub attributes: BasicAgent,
    pub factsheets: FactSheet,
    pub agent: Vec<Box<dyn SpacialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let attributes = BasicAgent {
            objective: "I am a managing agent, I can help you with your problem".to_string(),
            position: "Project Manager".to_string(),
            state: AgentState::Discovering,
            memory: vec![],
        };
        let project_description: String = ai_task_request(
            user_request,
            "Project Manager",
            get_function_string!(print_project_scope),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpacialFunctions>> = vec![];
        let factsheets = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            api_end_point_schema: None,
            backend_code: None,
        };
        Ok(Self {
            attributes,
            factsheets,
            agent: agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpacialFunctions>) {
        self.agent.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();
        for agent in &mut self.agent {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheets).await;
            let agent_info = agent.get_attributes_from_agents();
            dbg!(agent_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_managing_agent() {
        let usr_request = "I want to build a website".to_string();
        let mut managing_agent: ManagingAgent = ManagingAgent::new(usr_request.to_string())
            .await
            .expect("Error creating manager agent");

        managing_agent.execute_project().await;
        dbg!(managing_agent.factsheets);
    }
}
