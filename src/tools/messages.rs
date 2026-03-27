use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendMessageParams {
    /// Chat ID or @channel_username
    pub chat_id: String,
    /// Text of the message (1-4096 characters)
    pub text: String,
    /// Parse mode: HTML, Markdown, MarkdownV2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Disables link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<serde_json::Value>,
    /// Sends the message silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier for the target message thread (topic) of a forum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Description of the message to reply to (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// InlineKeyboardMarkup, ReplyKeyboardMarkup, etc. (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ForwardMessageParams {
    /// Target chat ID or @username
    pub chat_id: String,
    /// Source chat ID or @username
    pub from_chat_id: String,
    /// Message ID to forward
    pub message_id: i64,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Sends the message silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects forwarded message from saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ForwardMessagesParams {
    /// Target chat ID or @username
    pub chat_id: String,
    /// Source chat ID or @username
    pub from_chat_id: String,
    /// List of message IDs to forward (1-100)
    pub message_ids: Vec<i64>,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Sends the messages silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects messages from saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CopyMessageParams {
    /// Target chat ID or @username
    pub chat_id: String,
    /// Source chat ID or @username
    pub from_chat_id: String,
    /// Message ID to copy
    pub message_id: i64,
    /// New caption for the message (0-1024 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the new caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Sends the message silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects message from saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Description of the message to reply to (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// Reply markup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CopyMessagesParams {
    /// Target chat ID or @username
    pub chat_id: String,
    /// Source chat ID or @username
    pub from_chat_id: String,
    /// List of message IDs to copy (1-100)
    pub message_ids: Vec<i64>,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Sends the messages silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects messages from saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass True to copy without captions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendChatActionParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Action: typing, upload_photo, record_video, upload_video, record_voice, upload_voice, upload_document, choose_sticker, find_location, record_video_note, upload_video_note
    pub action: String,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMessageReactionParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID
    pub message_id: i64,
    /// JSON array of ReactionType objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<serde_json::Value>,
    /// Pass True for a big animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PinChatMessageParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID to pin
    pub message_id: i64,
    /// Pass True to pin silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Business connection ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UnpinChatMessageParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID to unpin (if not specified, the most recent pinned message is unpinned)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Business connection ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UnpinAllChatMessagesParams {
    /// Chat ID or @username
    pub chat_id: String,
}

#[tool_router(router = tool_router_messages, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Send a text message to a chat or channel")]
    async fn send_message(
        &self,
        params: Parameters<SendMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendMessage", &params.0).await
    }

    #[tool(description = "Forward a message from one chat to another")]
    async fn forward_message(
        &self,
        params: Parameters<ForwardMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("forwardMessage", &params.0).await
    }

    #[tool(description = "Forward multiple messages at once (1-100)")]
    async fn forward_messages(
        &self,
        params: Parameters<ForwardMessagesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("forwardMessages", &params.0).await
    }

    #[tool(description = "Copy a message (send without 'Forwarded from' header)")]
    async fn copy_message(
        &self,
        params: Parameters<CopyMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("copyMessage", &params.0).await
    }

    #[tool(description = "Copy multiple messages at once (1-100)")]
    async fn copy_messages(
        &self,
        params: Parameters<CopyMessagesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("copyMessages", &params.0).await
    }

    #[tool(description = "Send a chat action (typing, uploading, etc.)")]
    async fn send_chat_action(
        &self,
        params: Parameters<SendChatActionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("sendChatAction", &params.0).await
    }

    #[tool(description = "Set a reaction on a message (emoji or custom emoji)")]
    async fn set_message_reaction(
        &self,
        params: Parameters<SetMessageReactionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMessageReaction", &params.0).await
    }

    #[tool(description = "Pin a message in a chat")]
    async fn pin_chat_message(
        &self,
        params: Parameters<PinChatMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("pinChatMessage", &params.0).await
    }

    #[tool(description = "Unpin a message in a chat")]
    async fn unpin_chat_message(
        &self,
        params: Parameters<UnpinChatMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("unpinChatMessage", &params.0).await
    }

    #[tool(description = "Unpin all pinned messages in a chat")]
    async fn unpin_all_chat_messages(
        &self,
        params: Parameters<UnpinAllChatMessagesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("unpinAllChatMessages", &params.0).await
    }
}
