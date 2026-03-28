use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageTextParams {
    /// Chat ID or @username (required for non-inline messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID to edit (required for non-inline messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID (required for inline messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message
    pub text: String,
    /// Parse mode: HTML, Markdown, MarkdownV2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Link preview options (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<serde_json::Value>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageCaptionParams {
    /// Chat ID or @username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageMediaParams {
    /// Chat ID or @username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// InputMedia object (JSON)
    pub media: serde_json::Value,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageLiveLocationParams {
    /// Chat ID or @username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// The radius of uncertainty for the location (0-1500 meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction of movement (1-360 degrees)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Max distance for proximity alerts (1-100000 meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StopMessageLiveLocationParams {
    /// Chat ID or @username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageReplyMarkupParams {
    /// Chat ID or @username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Message ID to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Inline message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StopPollParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID of the poll
    pub message_id: i64,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeleteMessageParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID to delete
    pub message_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeleteMessagesParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// List of message IDs to delete (1-100)
    pub message_ids: Vec<i64>,
}

#[tool_router(router = tool_router_editing, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Edit text of a message")]
    async fn edit_message_text(
        &self,
        params: Parameters<EditMessageTextParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("editMessageText", &params.0).await
    }

    #[tool(description = "Edit caption of a message")]
    async fn edit_message_caption(
        &self,
        params: Parameters<EditMessageCaptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("editMessageCaption", &params.0).await
    }

    #[tool(description = "Edit media content of a message")]
    async fn edit_message_media(
        &self,
        params: Parameters<EditMessageMediaParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("editMessageMedia", &params.0).await
    }

    #[tool(description = "Edit a live location message")]
    async fn edit_message_live_location(
        &self,
        params: Parameters<EditMessageLiveLocationParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("editMessageLiveLocation", &params.0)
            .await
    }

    #[tool(description = "Stop updating a live location message")]
    async fn stop_message_live_location(
        &self,
        params: Parameters<StopMessageLiveLocationParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("stopMessageLiveLocation", &params.0)
            .await
    }

    #[tool(description = "Edit the reply markup of a message")]
    async fn edit_message_reply_markup(
        &self,
        params: Parameters<EditMessageReplyMarkupParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("editMessageReplyMarkup", &params.0)
            .await
    }

    #[tool(description = "Stop a poll sent by the bot")]
    async fn stop_poll(
        &self,
        params: Parameters<StopPollParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("stopPoll", &params.0).await
    }

    #[tool(description = "Delete a message")]
    async fn delete_message(
        &self,
        params: Parameters<DeleteMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteMessage", &params.0).await
    }

    #[tool(description = "Delete multiple messages at once (1-100)")]
    async fn delete_messages(
        &self,
        params: Parameters<DeleteMessagesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteMessages", &params.0).await
    }
}
