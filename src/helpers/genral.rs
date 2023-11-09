use std::{fmt::format, fs};

use actix_web::cookie::time::ext;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserializer};

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::command_line::PrintCommand;
const CODE_TEMPLATE_PATH: &str =
    "/Users/orpheasboutsarides/Learn/Rust/web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str = "/Users/orpheasboutsarides/Learn/Rust/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/Users/orpheasboutsarides/Learn/Rust/auto_chatGTP/src/schemas/api_schema.json";

pub fn extend_ai_function(ai_fun: fn(&str) -> &'static str, fun_input: &str) -> Message {
    let ai_fun_output = ai_fun(fun_input);
    dbg!(ai_fun_output);
    // Extend the string with the ai_fun_output
    let msg: String = format!(
        "Function {}, Instruction: You are a function printer. Only print results of function. Here is the input of the function {}. Print out what the function will return",
        ai_fun_output,
        fun_input
    );
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI fun
    let extend_message_func_msg = extend_ai_function(function_pass, &msg_context);
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get agent response
    let llm_response_res = call_gpt(vec![extend_message_func_msg.clone()]).await;
    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => {
            call_gpt(vec![extend_message_func_msg.clone()])
                .await
                .expect("Failed twice to call Open AI")
            // let err_msg = "Failed to get response from LLM";
            // PrintCommand::Issue.print_agent_message(agent_position, err_msg);
            // return err_msg.to_string();
        }
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    // Extend AI fun
    let llm_reponse =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T =
        serde_json::from_str(llm_reponse.as_str()).expect("Failed to decode response from LLM");
    decoded_response
}

pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let res = client.get(url).send().await?;
    Ok(res.status().as_u16())
}

pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

pub fn save_backend_code(contents: &str) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write code template");
}

pub fn save_api_endpoints(contents: &str) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, contents).expect("Failed to write API and points");
}

// /Users/orpheasboutsarides/Learn/Rust/web_template/src

#[cfg(test)]
mod tests {
    use crossterm::style::Stylize;

    use super::*;
    use crate::ai_functions::ai_fun_manager::{self, convert_user_input_to_goal};
    #[test]
    fn tests_extend_ai_fun() {
        let extend_mesg = extend_ai_function(convert_user_input_to_goal, "dymmy variable");
        assert_eq!(extend_mesg.role, "system ".to_string());
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_params = "Build me a website".to_string();

        let res = ai_task_request(
            ai_func_params,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;
        assert!(res.len() > 20);
    }
}
