use std::fmt::Debug;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::agent_basic::basic_agent::BasicAgent;

#[derive(Debug, PartialEq, Clone, Serialize, Copy, Deserialize)]
pub struct ProjectScope {
    pub is_crud_requierd: bool,
    pub is_crud_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RouteObject {
    pub is_route_dynamic: bool,
    pub method: bool,
    pub request_body: Value,
    pub response_body: Value,
    pub route: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_end_point_schema: Option<RouteObject>,
}

#[async_trait]
pub trait SpacialFunctions: Debug {
    fn get_attributes_from_agents(&self) -> &BasicAgent;
    async fn execute(
        &mut self,
        factsheets: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
