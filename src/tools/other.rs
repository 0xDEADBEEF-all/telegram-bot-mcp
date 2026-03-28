use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetPassportDataErrorsParams {
    /// User ID
    pub user_id: i64,
    /// JSON array of PassportElementError objects
    pub errors: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUserProfilePhotosParams {
    /// User ID
    pub user_id: i64,
    /// Offset of the first photo (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Max number of photos (1-100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetFileParams {
    /// File identifier
    pub file_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUserChatBoostsParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBusinessConnectionParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendGiftParams {
    /// User ID to send the gift to
    pub user_id: i64,
    /// Identifier of the gift to send
    pub gift_id: String,
    /// Text to show along with the gift (0-255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Parse mode for the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// Special entities in the text (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<serde_json::Value>,
}

#[tool_router(router = tool_router_other, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Inform Telegram Passport about errors in user-submitted data")]
    async fn set_passport_data_errors(
        &self,
        params: Parameters<SetPassportDataErrorsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setPassportDataErrors", &params.0)
            .await
    }

    #[tool(description = "Get a list of profile pictures of a user")]
    async fn get_user_profile_photos(
        &self,
        params: Parameters<GetUserProfilePhotosParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getUserProfilePhotos", &params.0)
            .await
    }

    #[tool(description = "Get basic info about a file and prepare it for downloading")]
    async fn get_file(
        &self,
        params: Parameters<GetFileParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getFile", &params.0).await
    }

    #[tool(description = "Get the list of boosts added to a chat by a user")]
    async fn get_user_chat_boosts(
        &self,
        params: Parameters<GetUserChatBoostsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getUserChatBoosts", &params.0).await
    }

    #[tool(description = "Get information about a business connection")]
    async fn get_business_connection(
        &self,
        params: Parameters<GetBusinessConnectionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getBusinessConnection", &params.0)
            .await
    }

    #[tool(description = "Get the list of gifts available for sending")]
    async fn get_available_gifts(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("getAvailableGifts").await
    }

    #[tool(description = "Send a gift to a user using Telegram Stars")]
    async fn send_gift(
        &self,
        params: Parameters<SendGiftParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("sendGift", &params.0).await
    }
}
