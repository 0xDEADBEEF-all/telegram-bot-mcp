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

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyProfilePhotoParams {
    /// Photo to set (InputProfilePhoto JSON)
    pub photo: serde_json::Value,
    /// Pass True to set a public photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RemoveMyProfilePhotoParams {
    /// Pass True to remove a public photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetUserEmojiStatusParams {
    /// User ID
    pub user_id: i64,
    /// Custom emoji identifier to set (empty to remove)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date for the emoji status (unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUserGiftsParams {
    /// User ID
    pub user_id: i64,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// Max number of gifts to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetChatGiftsParams {
    /// Chat ID
    pub chat_id: String,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// Max number of gifts to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ConvertGiftToStarsParams {
    /// Unique identifier of the business connection (if on behalf of business)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Identifier of the received gift
    pub owned_gift_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UpgradeGiftParams {
    /// Unique identifier of the business connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Identifier of the received gift to upgrade
    pub owned_gift_id: String,
    /// Pass True to keep the original sender's name hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_original_details: Option<bool>,
    /// New text for the gift (0-255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Parse mode for text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// Text entities (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TransferGiftParams {
    /// Unique identifier of the business connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// Identifier of the received gift to transfer
    pub owned_gift_id: String,
    /// User ID of the new owner
    pub new_owner_chat_id: i64,
    /// Number of Stars to pay for the transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GiftPremiumSubscriptionParams {
    /// User ID to gift Premium to
    pub user_id: i64,
    /// Number of months (3, 6, or 12)
    pub month_count: i64,
    /// Number of Stars to pay
    pub star_count: i64,
    /// Text to show along with the gift (0-255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Parse mode for text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// Text entities (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct VerifyUserParams {
    /// User ID to verify
    pub user_id: i64,
    /// Custom description for the verification (0-70 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct VerifyChatParams {
    /// Chat ID or @username to verify
    pub chat_id: String,
    /// Custom description for the verification (0-70 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RemoveUserVerificationParams {
    /// User ID to remove verification from
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RemoveChatVerificationParams {
    /// Chat ID or @username to remove verification from
    pub chat_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendChecklistParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Checklist title
    pub title: String,
    /// JSON array of InputChecklist items
    pub items: serde_json::Value,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Reply parameters (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// Reply markup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditMessageChecklistParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Message ID to edit
    pub message_id: i64,
    /// New checklist title
    pub title: String,
    /// JSON array of InputChecklist items
    pub items: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendMessageDraftParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Draft message content (InputMessageDraft JSON)
    pub draft: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatMemberTagParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID
    pub user_id: i64,
    /// Custom tag (0-12 characters, empty to remove)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetUserProfileAudiosParams {
    /// User ID
    pub user_id: i64,
    /// Offset of the first audio (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Max number of audios to return (1-100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ApproveSuggestedPostParams {
    /// Chat ID of the channel where the post was suggested
    pub chat_id: String,
    /// Message ID of the suggested post
    pub message_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct DeclineSuggestedPostParams {
    /// Chat ID of the channel where the post was suggested
    pub chat_id: String,
    /// Message ID of the suggested post
    pub message_id: i64,
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

    #[tool(description = "Set the bot's profile photo")]
    async fn set_my_profile_photo(
        &self,
        params: Parameters<SetMyProfilePhotoParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setMyProfilePhoto", &params.0)
            .await
    }

    #[tool(description = "Remove the bot's profile photo")]
    async fn remove_my_profile_photo(
        &self,
        params: Parameters<RemoveMyProfilePhotoParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("removeMyProfilePhoto", &params.0)
            .await
    }

    #[tool(description = "Set a custom emoji status for a user")]
    async fn set_user_emoji_status(
        &self,
        params: Parameters<SetUserEmojiStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setUserEmojiStatus", &params.0)
            .await
    }

    #[tool(description = "Get the gifts received by a user")]
    async fn get_user_gifts(
        &self,
        params: Parameters<GetUserGiftsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getUserGifts", &params.0).await
    }

    #[tool(description = "Get the gifts received by a chat")]
    async fn get_chat_gifts(
        &self,
        params: Parameters<GetChatGiftsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getChatGifts", &params.0).await
    }

    #[tool(description = "Convert a received gift to Telegram Stars")]
    async fn convert_gift_to_stars(
        &self,
        params: Parameters<ConvertGiftToStarsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("convertGiftToStars", &params.0)
            .await
    }

    #[tool(description = "Upgrade a received gift to a unique gift")]
    async fn upgrade_gift(
        &self,
        params: Parameters<UpgradeGiftParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("upgradeGift", &params.0).await
    }

    #[tool(description = "Transfer a received gift to another user")]
    async fn transfer_gift(
        &self,
        params: Parameters<TransferGiftParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("transferGift", &params.0).await
    }

    #[tool(description = "Gift a Telegram Premium subscription to a user")]
    async fn gift_premium_subscription(
        &self,
        params: Parameters<GiftPremiumSubscriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("giftPremiumSubscription", &params.0)
            .await
    }

    #[tool(description = "Verify a user on behalf of the bot's organization")]
    async fn verify_user(
        &self,
        params: Parameters<VerifyUserParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("verifyUser", &params.0).await
    }

    #[tool(description = "Verify a chat on behalf of the bot's organization")]
    async fn verify_chat(
        &self,
        params: Parameters<VerifyChatParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("verifyChat", &params.0).await
    }

    #[tool(description = "Remove verification from a user")]
    async fn remove_user_verification(
        &self,
        params: Parameters<RemoveUserVerificationParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("removeUserVerification", &params.0)
            .await
    }

    #[tool(description = "Remove verification from a chat")]
    async fn remove_chat_verification(
        &self,
        params: Parameters<RemoveChatVerificationParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("removeChatVerification", &params.0)
            .await
    }

    #[tool(description = "Send a checklist message")]
    async fn send_checklist(
        &self,
        params: Parameters<SendChecklistParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendChecklist", &params.0).await
    }

    #[tool(description = "Edit a checklist message")]
    async fn edit_message_checklist(
        &self,
        params: Parameters<EditMessageChecklistParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("editMessageChecklist", &params.0)
            .await
    }

    #[tool(description = "Send a message draft to a chat")]
    async fn send_message_draft(
        &self,
        params: Parameters<SendMessageDraftParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendMessageDraft", &params.0).await
    }

    #[tool(description = "Set a custom tag for a chat member (visible only to admins)")]
    async fn set_chat_member_tag(
        &self,
        params: Parameters<SetChatMemberTagParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setChatMemberTag", &params.0)
            .await
    }

    #[tool(description = "Get the bot's Star balance")]
    async fn get_my_star_balance(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("getMyStarBalance").await
    }

    #[tool(description = "Get a list of profile audios of a user")]
    async fn get_user_profile_audios(
        &self,
        params: Parameters<GetUserProfileAudiosParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getUserProfileAudios", &params.0)
            .await
    }

    #[tool(description = "Approve a suggested post in a channel")]
    async fn approve_suggested_post(
        &self,
        params: Parameters<ApproveSuggestedPostParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("approveSuggestedPost", &params.0)
            .await
    }

    #[tool(description = "Decline a suggested post in a channel")]
    async fn decline_suggested_post(
        &self,
        params: Parameters<DeclineSuggestedPostParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("declineSuggestedPost", &params.0)
            .await
    }
}
