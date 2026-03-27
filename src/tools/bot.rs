use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyCommandsParams {
    /// JSON array of BotCommand objects, e.g. [{"command":"start","description":"Start the bot"}]
    pub commands: serde_json::Value,
    /// Scope of commands (JSON object), e.g. {"type":"default"}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>,
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ScopeParams {
    /// Scope of commands (JSON object), e.g. {"type":"default"}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>,
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyNameParams {
    /// New bot name (0-64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LanguageCodeParams {
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyDescriptionParams {
    /// New bot description (0-512 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyShortDescriptionParams {
    /// New short description (0-120 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    /// Two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ChatMenuButtonParams {
    /// Unique identifier for the target private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetChatMenuButtonParams {
    /// Unique identifier for the target private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// MenuButton object (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetMyDefaultAdminRightsParams {
    /// ChatAdministratorRights object (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<serde_json::Value>,
    /// Pass True for channels, False for groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetMyDefaultAdminRightsParams {
    /// Pass True for channels, False for groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[tool_router(router = tool_router_bot, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Get basic information about the bot (username, id, name)")]
    async fn get_me(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("getMe").await
    }

    #[tool(description = "Log out from the cloud Bot API server")]
    async fn log_out(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("logOut").await
    }

    #[tool(description = "Close the bot instance before moving it to a new server")]
    async fn close(&self) -> Result<CallToolResult, McpError> {
        self.api.call_method_no_params("close").await
    }

    #[tool(description = "Set the bot's command list shown in the menu")]
    async fn set_my_commands(
        &self,
        params: Parameters<SetMyCommandsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMyCommands", &params.0).await
    }

    #[tool(description = "Delete the bot's command list")]
    async fn delete_my_commands(
        &self,
        params: Parameters<ScopeParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteMyCommands", &params.0).await
    }

    #[tool(description = "Get the bot's command list")]
    async fn get_my_commands(
        &self,
        params: Parameters<ScopeParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getMyCommands", &params.0).await
    }

    #[tool(description = "Set the bot's name")]
    async fn set_my_name(
        &self,
        params: Parameters<SetMyNameParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMyName", &params.0).await
    }

    #[tool(description = "Get the bot's name")]
    async fn get_my_name(
        &self,
        params: Parameters<LanguageCodeParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getMyName", &params.0).await
    }

    #[tool(description = "Set the bot's description (shown in empty chat)")]
    async fn set_my_description(
        &self,
        params: Parameters<SetMyDescriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMyDescription", &params.0).await
    }

    #[tool(description = "Get the bot's description")]
    async fn get_my_description(
        &self,
        params: Parameters<LanguageCodeParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getMyDescription", &params.0).await
    }

    #[tool(description = "Set the bot's short description (shown on profile)")]
    async fn set_my_short_description(
        &self,
        params: Parameters<SetMyShortDescriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMyShortDescription", &params.0).await
    }

    #[tool(description = "Get the bot's short description")]
    async fn get_my_short_description(
        &self,
        params: Parameters<LanguageCodeParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getMyShortDescription", &params.0).await
    }

    #[tool(description = "Set the bot's menu button in a private chat")]
    async fn set_chat_menu_button(
        &self,
        params: Parameters<SetChatMenuButtonParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setChatMenuButton", &params.0).await
    }

    #[tool(description = "Get the bot's menu button in a private chat")]
    async fn get_chat_menu_button(
        &self,
        params: Parameters<ChatMenuButtonParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getChatMenuButton", &params.0).await
    }

    #[tool(description = "Set default administrator rights for the bot")]
    async fn set_my_default_administrator_rights(
        &self,
        params: Parameters<SetMyDefaultAdminRightsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setMyDefaultAdministratorRights", &params.0).await
    }

    #[tool(description = "Get default administrator rights of the bot")]
    async fn get_my_default_administrator_rights(
        &self,
        params: Parameters<GetMyDefaultAdminRightsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getMyDefaultAdministratorRights", &params.0).await
    }
}
