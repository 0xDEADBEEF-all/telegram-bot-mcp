use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ReadBusinessMessageParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Chat ID in the business account
    pub chat_id: i64,
    /// Message ID to mark as read
    pub message_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeleteBusinessMessagesParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Chat ID in the business account
    pub chat_id: i64,
    /// List of message IDs to delete (1-100)
    pub message_ids: Vec<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetBusinessAccountNameParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// New first name (1-64 characters)
    pub first_name: String,
    /// New last name (0-64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetBusinessAccountUsernameParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// New username (0-32 characters, empty to remove)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetBusinessAccountBioParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// New bio (0-140 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetBusinessAccountProfilePhotoParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Photo to set (InputProfilePhoto JSON)
    pub photo: serde_json::Value,
    /// Pass True to set a public photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RemoveBusinessAccountProfilePhotoParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass True to remove a public photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetBusinessAccountGiftSettingsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass True to show gift button in the account
    pub show_gift_button: bool,
    /// AcceptedGiftTypes object (JSON)
    pub accepted_gift_types: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct BusinessConnectionIdParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransferBusinessAccountStarsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Number of Stars to transfer
    pub star_count: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetBusinessAccountGiftsParams {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass True to exclude unsaved gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    /// Pass True to exclude saved gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    /// Pass True to exclude gifts that can be converted to Stars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass True to exclude limited gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited: Option<bool>,
    /// Pass True to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Sort order: by_date or by_price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// Max number of gifts to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[tool_router(router = tool_router_business, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Mark a message as read in a business account chat")]
    async fn read_business_message(
        &self,
        params: Parameters<ReadBusinessMessageParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("readBusinessMessage", &params.0)
            .await
    }

    #[tool(description = "Delete messages in a business account chat")]
    async fn delete_business_messages(
        &self,
        params: Parameters<DeleteBusinessMessagesParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("deleteBusinessMessages", &params.0)
            .await
    }

    #[tool(description = "Set the name of a business account")]
    async fn set_business_account_name(
        &self,
        params: Parameters<SetBusinessAccountNameParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setBusinessAccountName", &params.0)
            .await
    }

    #[tool(description = "Set the username of a business account")]
    async fn set_business_account_username(
        &self,
        params: Parameters<SetBusinessAccountUsernameParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setBusinessAccountUsername", &params.0)
            .await
    }

    #[tool(description = "Set the bio of a business account")]
    async fn set_business_account_bio(
        &self,
        params: Parameters<SetBusinessAccountBioParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setBusinessAccountBio", &params.0)
            .await
    }

    #[tool(description = "Set the profile photo of a business account")]
    async fn set_business_account_profile_photo(
        &self,
        params: Parameters<SetBusinessAccountProfilePhotoParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setBusinessAccountProfilePhoto", &params.0)
            .await
    }

    #[tool(description = "Remove the profile photo of a business account")]
    async fn remove_business_account_profile_photo(
        &self,
        params: Parameters<RemoveBusinessAccountProfilePhotoParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("removeBusinessAccountProfilePhoto", &params.0)
            .await
    }

    #[tool(description = "Set the gift settings of a business account")]
    async fn set_business_account_gift_settings(
        &self,
        params: Parameters<SetBusinessAccountGiftSettingsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setBusinessAccountGiftSettings", &params.0)
            .await
    }

    #[tool(description = "Get the Star balance of a business account")]
    async fn get_business_account_star_balance(
        &self,
        params: Parameters<BusinessConnectionIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getBusinessAccountStarBalance", &params.0)
            .await
    }

    #[tool(description = "Transfer Stars from the bot to a business account")]
    async fn transfer_business_account_stars(
        &self,
        params: Parameters<TransferBusinessAccountStarsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("transferBusinessAccountStars", &params.0)
            .await
    }

    #[tool(description = "Get the gifts received by a business account")]
    async fn get_business_account_gifts(
        &self,
        params: Parameters<GetBusinessAccountGiftsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getBusinessAccountGifts", &params.0)
            .await
    }
}
