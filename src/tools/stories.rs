use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PostStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Content of the story (InputStoryContent JSON)
    pub content: serde_json::Value,
    /// Period in seconds after which the story will be deleted (6h=21600, 12h=43200, 24h=86400, 48h=172800)
    pub active_period: i64,
    /// Caption (0-2048 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Story areas (JSON array of StoryArea)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<serde_json::Value>,
    /// Pass True to post the story to the main profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    /// Pass True to protect the story from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to edit
    pub story_id: i64,
    /// New content (InputStoryContent JSON)
    pub content: serde_json::Value,
    /// New caption (0-2048 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// New story areas (JSON array of StoryArea)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeleteStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to delete
    pub story_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RepostStoryParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Chat ID of the story's original sender
    pub from_chat_id: i64,
    /// Story ID to repost
    pub story_id: i64,
}

#[tool_router(router = tool_router_stories, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Post a story on behalf of a business account")]
    async fn post_story(
        &self,
        params: Parameters<PostStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("postStory", &params.0).await
    }

    #[tool(description = "Edit a story posted by a business account")]
    async fn edit_story(
        &self,
        params: Parameters<EditStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("editStory", &params.0).await
    }

    #[tool(description = "Delete a story posted by a business account")]
    async fn delete_story(
        &self,
        params: Parameters<DeleteStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteStory", &params.0).await
    }

    #[tool(description = "Repost a story on behalf of a business account")]
    async fn repost_story(
        &self,
        params: Parameters<RepostStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("repostStory", &params.0).await
    }
}
