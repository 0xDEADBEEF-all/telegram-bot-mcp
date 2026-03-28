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
pub struct GetChatMemberParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct BanChatMemberParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID to ban
    pub user_id: i64,
    /// Unix time when the user will be unbanned (0 or absent = forever)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass True to delete all messages from the user in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UnbanChatMemberParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID to unban
    pub user_id: i64,
    /// True = do nothing if user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RestrictChatMemberParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID to restrict
    pub user_id: i64,
    /// ChatPermissions object (JSON)
    pub permissions: serde_json::Value,
    /// Pass True to use independent chat permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    /// Unix time when restrictions will be lifted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PromoteChatMemberParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID to promote
    pub user_id: i64,
    /// Can change chat info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Can post messages (channels only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Can edit messages (channels only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Can delete messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Can invite users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Can restrict members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Can pin messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Can promote other members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Can manage the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Can manage topics (supergroups)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Can post stories (channels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// Can edit stories (channels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// Can delete stories (channels)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// Is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatAdminTitleParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID
    pub user_id: i64,
    /// Custom admin title (0-16 characters, emoji not allowed)
    pub custom_title: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct BanChatSenderChatParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Sender chat ID to ban
    pub sender_chat_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatPermissionsParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// ChatPermissions object (JSON)
    pub permissions: serde_json::Value,
    /// Pass True to use independent chat permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateChatInviteLinkParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Invite link name (0-32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unix time when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Max uses of the link (0 = unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True if users need admin approval to join
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditChatInviteLinkParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// The invite link to edit
    pub invite_link: String,
    /// New name (0-32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unix time when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Max uses (0 = unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True if users need admin approval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RevokeChatInviteLinkParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// The invite link to revoke
    pub invite_link: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateChatSubscriptionInviteLinkParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Invite link name (0-32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of seconds the subscription will be active (currently 2592000 = 30 days)
    pub subscription_period: i64,
    /// Number of Telegram Stars a user must pay to join
    pub subscription_price: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditChatSubscriptionInviteLinkParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// The invite link to edit
    pub invite_link: String,
    /// New name (0-32 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ChatJoinRequestParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// User ID
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatPhotoParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// New chat photo (file_id or attach:// URI)
    pub photo: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatTitleParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// New chat title (1-128 characters)
    pub title: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatDescriptionParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// New description (0-255 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatStickerSetParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Name of the sticker set
    pub sticker_set_name: String,
}

#[tool_router(router = tool_router_chats, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Get up-to-date information about a chat")]
    async fn get_chat(&self, params: Parameters<ChatIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("getChat", &params.0).await
    }

    #[tool(description = "Get a list of administrators in a chat")]
    async fn get_chat_administrators(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("getChatAdministrators", &params.0)
            .await
    }

    #[tool(description = "Get the number of members in a chat")]
    async fn get_chat_member_count(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getChatMemberCount", &params.0).await
    }

    #[tool(description = "Get information about a member of a chat")]
    async fn get_chat_member(
        &self,
        params: Parameters<GetChatMemberParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getChatMember", &params.0).await
    }

    #[tool(description = "Ban a user in a group/supergroup/channel")]
    async fn ban_chat_member(
        &self,
        params: Parameters<BanChatMemberParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("banChatMember", &params.0).await
    }

    #[tool(description = "Unban a previously banned user")]
    async fn unban_chat_member(
        &self,
        params: Parameters<UnbanChatMemberParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("unbanChatMember", &params.0)
            .await
    }

    #[tool(description = "Restrict a user in a supergroup")]
    async fn restrict_chat_member(
        &self,
        params: Parameters<RestrictChatMemberParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("restrictChatMember", &params.0)
            .await
    }

    #[tool(description = "Promote or demote a user in a supergroup/channel")]
    async fn promote_chat_member(
        &self,
        params: Parameters<PromoteChatMemberParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("promoteChatMember", &params.0)
            .await
    }

    #[tool(description = "Set a custom title for an administrator")]
    async fn set_chat_administrator_custom_title(
        &self,
        params: Parameters<SetChatAdminTitleParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setChatAdministratorCustomTitle", &params.0)
            .await
    }

    #[tool(description = "Ban a channel chat in a supergroup/channel")]
    async fn ban_chat_sender_chat(
        &self,
        params: Parameters<BanChatSenderChatParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("banChatSenderChat", &params.0)
            .await
    }

    #[tool(description = "Unban a previously banned channel chat")]
    async fn unban_chat_sender_chat(
        &self,
        params: Parameters<BanChatSenderChatParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("unbanChatSenderChat", &params.0)
            .await
    }

    #[tool(description = "Set default chat permissions for all members")]
    async fn set_chat_permissions(
        &self,
        params: Parameters<SetChatPermissionsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setChatPermissions", &params.0)
            .await
    }

    #[tool(description = "Generate a new primary invite link (old link is revoked)")]
    async fn export_chat_invite_link(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("exportChatInviteLink", &params.0)
            .await
    }

    #[tool(description = "Create an additional invite link for a chat")]
    async fn create_chat_invite_link(
        &self,
        params: Parameters<CreateChatInviteLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("createChatInviteLink", &params.0)
            .await
    }

    #[tool(description = "Edit an existing invite link")]
    async fn edit_chat_invite_link(
        &self,
        params: Parameters<EditChatInviteLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("editChatInviteLink", &params.0).await
    }

    #[tool(description = "Revoke an invite link")]
    async fn revoke_chat_invite_link(
        &self,
        params: Parameters<RevokeChatInviteLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("revokeChatInviteLink", &params.0)
            .await
    }

    #[tool(description = "Create a subscription invite link for a channel")]
    async fn create_chat_subscription_invite_link(
        &self,
        params: Parameters<CreateChatSubscriptionInviteLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("createChatSubscriptionInviteLink", &params.0)
            .await
    }

    #[tool(description = "Edit a subscription invite link")]
    async fn edit_chat_subscription_invite_link(
        &self,
        params: Parameters<EditChatSubscriptionInviteLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method("editChatSubscriptionInviteLink", &params.0)
            .await
    }

    #[tool(description = "Approve a chat join request")]
    async fn approve_chat_join_request(
        &self,
        params: Parameters<ChatJoinRequestParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("approveChatJoinRequest", &params.0)
            .await
    }

    #[tool(description = "Decline a chat join request")]
    async fn decline_chat_join_request(
        &self,
        params: Parameters<ChatJoinRequestParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("declineChatJoinRequest", &params.0)
            .await
    }

    #[tool(description = "Set a new chat photo")]
    async fn set_chat_photo(
        &self,
        params: Parameters<SetChatPhotoParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setChatPhoto", &params.0).await
    }

    #[tool(description = "Delete a chat photo")]
    async fn delete_chat_photo(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("deleteChatPhoto", &params.0)
            .await
    }

    #[tool(description = "Set a new chat title")]
    async fn set_chat_title(
        &self,
        params: Parameters<SetChatTitleParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setChatTitle", &params.0).await
    }

    #[tool(description = "Set the chat description")]
    async fn set_chat_description(
        &self,
        params: Parameters<SetChatDescriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setChatDescription", &params.0)
            .await
    }

    #[tool(description = "Leave a group, supergroup, or channel")]
    async fn leave_chat(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("leaveChat", &params.0).await
    }

    #[tool(description = "Set a group sticker set")]
    async fn set_chat_sticker_set(
        &self,
        params: Parameters<SetChatStickerSetParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("setChatStickerSet", &params.0)
            .await
    }

    #[tool(description = "Delete a group sticker set")]
    async fn delete_chat_sticker_set(
        &self,
        params: Parameters<ChatIdParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("deleteChatStickerSet", &params.0)
            .await
    }
}
