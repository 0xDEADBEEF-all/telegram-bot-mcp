use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUpdatesParams {
    /// Identifier of the first update to be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Max number of updates (1-100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling (0 = short polling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// List of update types to receive (JSON array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetWebhookParams {
    /// HTTPS URL to send updates to
    pub url: String,
    /// Upload server certificate (PEM, as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// Fixed IP address for webhook requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Max allowed connections (1-100, default 40)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// List of update types to receive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    /// Secret token for X-Telegram-Bot-Api-Secret-Token header (1-256 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeleteWebhookParams {
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

#[tool_router(router = tool_router_updates, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Get incoming updates via long polling (only if webhook is not set)")]
    async fn get_updates(
        &self,
        params: Parameters<GetUpdatesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getUpdates", &params.0).await
    }

    #[tool(description = "Set a webhook URL for receiving updates")]
    async fn set_webhook(
        &self,
        params: Parameters<SetWebhookParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setWebhook", &params.0).await
    }

    #[tool(description = "Remove the webhook integration")]
    async fn delete_webhook(
        &self,
        params: Parameters<DeleteWebhookParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteWebhook", &params.0).await
    }

    #[tool(description = "Get current webhook status")]
    async fn get_webhook_info(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("getWebhookInfo").await
    }
}
