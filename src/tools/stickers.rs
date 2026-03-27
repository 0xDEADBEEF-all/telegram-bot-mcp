use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendStickerParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Sticker: file_id, HTTP URL, or attach:// URI
    pub sticker: String,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Reply parameters (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// Reply markup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetStickerSetParams {
    /// Name of the sticker set
    pub name: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetCustomEmojiStickersParams {
    /// List of custom emoji identifiers (1-200)
    pub custom_emoji_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UploadStickerFileParams {
    /// User ID of the sticker set owner
    pub user_id: i64,
    /// Sticker file (InputFile as attach:// URI)
    pub sticker: String,
    /// Format: "static", "animated", or "video"
    pub sticker_format: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateNewStickerSetParams {
    /// User ID of the sticker set owner
    pub user_id: i64,
    /// Short name of the sticker set (1-64 characters)
    pub name: String,
    /// Sticker set title (1-64 characters)
    pub title: String,
    /// JSON array of InputSticker objects
    pub stickers: serde_json::Value,
    /// Type: "regular", "mask", or "custom_emoji"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    /// True if stickers must be repainted by user color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AddStickerToSetParams {
    /// User ID of the sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// InputSticker object (JSON)
    pub sticker: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerPositionParams {
    /// File identifier of the sticker
    pub sticker: String,
    /// New position (0-based)
    pub position: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StickerFileIdParams {
    /// File identifier of the sticker
    pub sticker: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ReplaceStickerInSetParams {
    /// User ID of the sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// File identifier of the sticker to replace
    pub old_sticker: String,
    /// InputSticker object (JSON)
    pub sticker: serde_json::Value,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StickerSetNameParams {
    /// Sticker set name
    pub name: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerSetTitleParams {
    /// Sticker set name
    pub name: String,
    /// New title (1-64 characters)
    pub title: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerSetThumbnailParams {
    /// Sticker set name
    pub name: String,
    /// User ID of the set owner
    pub user_id: i64,
    /// Thumbnail (file_id, URL, or attach://) or pass empty string to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Format: "static", "animated", or "video"
    pub format: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    /// Sticker set name
    pub name: String,
    /// Custom emoji identifier, or omit to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerEmojiListParams {
    /// File identifier of the sticker
    pub sticker: String,
    /// List of 1-20 emoji
    pub emoji_list: Vec<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerKeywordsParams {
    /// File identifier of the sticker
    pub sticker: String,
    /// List of 0-20 keywords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SetStickerMaskPositionParams {
    /// File identifier of the sticker
    pub sticker: String,
    /// MaskPosition object (JSON), or omit to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<serde_json::Value>,
}

#[tool_router(router = tool_router_stickers, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Send a sticker")]
    async fn send_sticker(&self, params: Parameters<SendStickerParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendSticker", &params.0).await
    }

    #[tool(description = "Get a sticker set by name")]
    async fn get_sticker_set(&self, params: Parameters<GetStickerSetParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("getStickerSet", &params.0).await
    }

    #[tool(description = "Get info about custom emoji stickers by their identifiers")]
    async fn get_custom_emoji_stickers(&self, params: Parameters<GetCustomEmojiStickersParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("getCustomEmojiStickers", &params.0).await
    }

    #[tool(description = "Upload a sticker file for later use in createNewStickerSet/addStickerToSet")]
    async fn upload_sticker_file(&self, params: Parameters<UploadStickerFileParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("uploadStickerFile", &params.0).await
    }

    #[tool(description = "Create a new sticker set owned by a user")]
    async fn create_new_sticker_set(&self, params: Parameters<CreateNewStickerSetParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("createNewStickerSet", &params.0).await
    }

    #[tool(description = "Add a sticker to a set")]
    async fn add_sticker_to_set(&self, params: Parameters<AddStickerToSetParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("addStickerToSet", &params.0).await
    }

    #[tool(description = "Move a sticker in a set to a specific position")]
    async fn set_sticker_position_in_set(&self, params: Parameters<SetStickerPositionParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerPositionInSet", &params.0).await
    }

    #[tool(description = "Delete a sticker from a set")]
    async fn delete_sticker_from_set(&self, params: Parameters<StickerFileIdParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteStickerFromSet", &params.0).await
    }

    #[tool(description = "Replace an existing sticker in a set with a new one")]
    async fn replace_sticker_in_set(&self, params: Parameters<ReplaceStickerInSetParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("replaceStickerInSet", &params.0).await
    }

    #[tool(description = "Set the title of a sticker set")]
    async fn set_sticker_set_title(&self, params: Parameters<SetStickerSetTitleParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerSetTitle", &params.0).await
    }

    #[tool(description = "Set the thumbnail of a regular or mask sticker set")]
    async fn set_sticker_set_thumbnail(&self, params: Parameters<SetStickerSetThumbnailParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerSetThumbnail", &params.0).await
    }

    #[tool(description = "Set the thumbnail of a custom emoji sticker set")]
    async fn set_custom_emoji_sticker_set_thumbnail(&self, params: Parameters<SetCustomEmojiStickerSetThumbnailParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setCustomEmojiStickerSetThumbnail", &params.0).await
    }

    #[tool(description = "Delete a sticker set")]
    async fn delete_sticker_set(&self, params: Parameters<StickerSetNameParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("deleteStickerSet", &params.0).await
    }

    #[tool(description = "Set the emoji list for a sticker")]
    async fn set_sticker_emoji_list(&self, params: Parameters<SetStickerEmojiListParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerEmojiList", &params.0).await
    }

    #[tool(description = "Set search keywords for a sticker")]
    async fn set_sticker_keywords(&self, params: Parameters<SetStickerKeywordsParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerKeywords", &params.0).await
    }

    #[tool(description = "Set the mask position of a mask sticker")]
    async fn set_sticker_mask_position(&self, params: Parameters<SetStickerMaskPositionParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method_bool("setStickerMaskPosition", &params.0).await
    }
}
