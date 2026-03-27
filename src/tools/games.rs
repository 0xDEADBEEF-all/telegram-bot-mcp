use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendGameParams {
    /// Chat ID (must be integer)
    pub chat_id: i64,
    /// Short name of the game (as registered with @BotFather)
    pub game_short_name: String,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Reply parameters (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetGameScoreParams {
    /// User ID
    pub user_id: i64,
    /// New score (non-negative)
    pub score: i64,
    /// True if high score is allowed to decrease
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// True to not show notification about the score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Chat ID (required if inline_message_id is not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Message ID (required if inline_message_id is not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetGameHighScoresParams {
    /// User ID
    pub user_id: i64,
    /// Chat ID (required if inline_message_id is not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Message ID (required if inline_message_id is not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[tool_router(router = tool_router_games, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Send a game")]
    async fn send_game(&self, params: Parameters<SendGameParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendGame", &params.0).await
    }

    #[tool(description = "Set the score of a user in a game")]
    async fn set_game_score(&self, params: Parameters<SetGameScoreParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("setGameScore", &params.0).await
    }

    #[tool(description = "Get high score table for a game")]
    async fn get_game_high_scores(&self, params: Parameters<GetGameHighScoresParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("getGameHighScores", &params.0).await
    }
}
