use std::fmt::format;

use crate::{
    ai_functions::{
        ai_fun_architech::print_project_scope,
        ai_fun_backend::{print_backend_webserver_code, print_improved_webserver_code},
    },
    helpers::genral::{
        ai_task_request, ai_task_request_decoded, read_code_template_contents, save_backend_code,
    },
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};

use super::agent_trait::{FactSheet, ProjectScope};

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
            memory: vec![],
        };
        Self {
            attributes,
            bugs_errors: None,
            bug_count: 0,
        }
    }

    async fn call_init_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_tempalte_srt: String = read_code_template_contents();

        // Concatenate the code template with the factsheet
        let msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_tempalte_srt, factsheet.project_description,
        );
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response)
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        // Concatenate the code template with the factsheet
        let msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet,
        );
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response)
    }
}
