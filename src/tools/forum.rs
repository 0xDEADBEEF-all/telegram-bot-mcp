use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ChatIdParams {
    /// Chat ID or @username
    pub chat_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateForumTopicParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Topic name (1-128 characters)
    pub name: String,
    /// Color of the topic icon: 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), 16478047 (0xFB6F5F)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    /// Custom emoji identifier for the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditForumTopicParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Unique identifier for the target message thread
    pub message_thread_id: i64,
    /// New topic name (0-128 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New custom emoji identifier for the icon (pass empty string to remove)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ForumTopicParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Unique identifier for the target message thread
    pub message_thread_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditGeneralForumTopicParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// New topic name (1-128 characters)
    pub name: String,
}

#[tool_router(router = tool_router_forum, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Get custom emoji stickers for forum topic icons")]
    async fn get_forum_topic_icon_stickers(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("getForumTopicIconStickers").await
    }

    #[tool(description = "Create a topic in a forum supergroup")]
    async fn create_forum_topic(&self, params: Parameters<CreateForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("createForumTopic", &params.0).await
    }

    #[tool(description = "Edit name and icon of a forum topic")]
    async fn edit_forum_topic(&self, params: Parameters<EditForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("editForumTopic", &params.0).await
    }

    #[tool(description = "Close a forum topic")]
    async fn close_forum_topic(&self, params: Parameters<ForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("closeForumTopic", &params.0).await
    }

    #[tool(description = "Reopen a closed forum topic")]
    async fn reopen_forum_topic(&self, params: Parameters<ForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("reopenForumTopic", &params.0).await
    }

    #[tool(description = "Delete a forum topic and all its messages")]
    async fn delete_forum_topic(&self, params: Parameters<ForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteForumTopic", &params.0).await
    }

    #[tool(description = "Clear the list of pinned messages in a forum topic")]
    async fn unpin_all_forum_topic_messages(&self, params: Parameters<ForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("unpinAllForumTopicMessages", &params.0).await
    }

    #[tool(description = "Edit the name of the 'General' forum topic")]
    async fn edit_general_forum_topic(&self, params: Parameters<EditGeneralForumTopicParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("editGeneralForumTopic", &params.0).await
    }

    #[tool(description = "Close the 'General' forum topic")]
    async fn close_general_forum_topic(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("closeGeneralForumTopic", &params.0).await
    }

    #[tool(description = "Reopen the 'General' forum topic")]
    async fn reopen_general_forum_topic(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("reopenGeneralForumTopic", &params.0).await
    }

    #[tool(description = "Hide the 'General' forum topic")]
    async fn hide_general_forum_topic(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("hideGeneralForumTopic", &params.0).await
    }

    #[tool(description = "Unhide the 'General' forum topic")]
    async fn unhide_general_forum_topic(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("unhideGeneralForumTopic", &params.0).await
    }

    #[tool(description = "Clear pinned messages in the 'General' forum topic")]
    async fn unpin_all_general_forum_topic_messages(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("unpinAllGeneralForumTopicMessages", &params.0).await
    }
}
