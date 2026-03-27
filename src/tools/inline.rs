use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AnswerCallbackQueryParams {
    /// Unique identifier for the callback query
    pub callback_query_id: String,
    /// Text to show as notification or alert (0-200 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// True to show an alert instead of a notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL to open (for game bots)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Max time in seconds the result can be cached (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AnswerInlineQueryParams {
    /// Unique identifier for the inline query
    pub inline_query_id: String,
    /// JSON array of InlineQueryResult objects (max 50)
    pub results: serde_json::Value,
    /// Max time in seconds the results can be cached (default 300)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    /// True if results are personal (not cached for all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// InlineQueryResultsButton (JSON) — button shown above inline results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AnswerWebAppQueryParams {
    /// Unique identifier for the answered query
    pub web_app_query_id: String,
    /// InlineQueryResult object (JSON)
    pub result: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SavePreparedInlineMessageParams {
    /// User ID of the user that allowed the bot to send the message
    pub user_id: i64,
    /// InlineQueryResult object (JSON)
    pub result: serde_json::Value,
    /// True to allow sending to private chats with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// True to allow sending to private chats with users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// True to allow sending to group/supergroup chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// True to allow sending to channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

#[tool_router(router = tool_router_inline, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Answer a callback query from an inline keyboard button")]
    async fn answer_callback_query(&self, params: Parameters<AnswerCallbackQueryParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("answerCallbackQuery", &params.0).await
    }

    #[tool(description = "Answer an inline query with results")]
    async fn answer_inline_query(&self, params: Parameters<AnswerInlineQueryParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("answerInlineQuery", &params.0).await
    }

    #[tool(description = "Answer a Web App query")]
    async fn answer_web_app_query(&self, params: Parameters<AnswerWebAppQueryParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("answerWebAppQuery", &params.0).await
    }

    #[tool(description = "Save a prepared inline message for a user to send")]
    async fn save_prepared_inline_message(&self, params: Parameters<SavePreparedInlineMessageParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("savePreparedInlineMessage", &params.0).await
    }
}
