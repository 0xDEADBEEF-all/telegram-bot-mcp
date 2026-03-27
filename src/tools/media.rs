use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendPhotoParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Photo to send: file_id, HTTP URL, or attach:// URI
    pub photo: String,
    /// Photo caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode: HTML, Markdown, MarkdownV2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Pass True for spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Target message thread ID
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
pub struct SendAudioParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Audio file: file_id, HTTP URL, or attach:// URI
    pub audio: String,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
pub struct SendDocumentParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Document: file_id, HTTP URL, or attach:// URI
    pub document: String,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Disables content type detection for uploaded files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
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
pub struct SendVideoParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Video: file_id, HTTP URL, or attach:// URI
    pub video: String,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Pass True for spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
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
pub struct SendAnimationParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Animation (GIF/MPEG4): file_id, HTTP URL, or attach:// URI
    pub animation: String,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Pass True for spoiler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
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
pub struct SendVoiceParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Voice audio (OGG/OPUS): file_id, HTTP URL, or attach:// URI
    pub voice: String,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
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
pub struct SendVideoNoteParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Video note (rounded square MPEG4): file_id or attach:// URI
    pub video_note: String,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width and height (diameter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
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
pub struct SendPaidMediaParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Number of Telegram Stars to charge
    pub star_count: i64,
    /// JSON array of InputPaidMedia objects
    pub media: serde_json::Value,
    /// Caption (0-1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
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
pub struct SendMediaGroupParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// JSON array of InputMedia objects (2-10 items)
    pub media: serde_json::Value,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Reply parameters (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendLocationParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Radius of uncertainty (0-1500 meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds for live location (60-86400)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Direction of movement (1-360)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Max distance for proximity alerts (1-100000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
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
pub struct SendVenueParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
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
pub struct SendContactParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data in vCard format (0-2048 bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
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
pub struct SendPollParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Poll question (1-300 characters)
    pub question: String,
    /// JSON array of InputPollOption objects (2-10 items)
    pub options: serde_json::Value,
    /// True for anonymous poll (default True)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type: "quiz" or "regular" (default "regular")
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub poll_type: Option<String>,
    /// True if multiple answers are allowed (regular polls only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based ID of the correct answer (quiz mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Explanation for the correct answer (0-200 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Parse mode for explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// Auto-close time (5-600 seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Unix timestamp when poll will be closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// True if the poll is immediately closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
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
pub struct SendDiceParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Emoji: 🎲 🎯 🏀 ⚽ 🎳 🎰 (default 🎲)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
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

#[tool_router(router = tool_router_media, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Send a photo to a chat")]
    async fn send_photo(&self, params: Parameters<SendPhotoParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendPhoto", &params.0).await
    }

    #[tool(description = "Send an audio file (mp3, etc.)")]
    async fn send_audio(&self, params: Parameters<SendAudioParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendAudio", &params.0).await
    }

    #[tool(description = "Send a general file/document")]
    async fn send_document(&self, params: Parameters<SendDocumentParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendDocument", &params.0).await
    }

    #[tool(description = "Send a video file")]
    async fn send_video(&self, params: Parameters<SendVideoParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendVideo", &params.0).await
    }

    #[tool(description = "Send an animation (GIF or MPEG4 without sound)")]
    async fn send_animation(&self, params: Parameters<SendAnimationParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendAnimation", &params.0).await
    }

    #[tool(description = "Send a voice message (OGG encoded with OPUS)")]
    async fn send_voice(&self, params: Parameters<SendVoiceParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendVoice", &params.0).await
    }

    #[tool(description = "Send a video note (rounded square MPEG4 video, up to 1 min)")]
    async fn send_video_note(&self, params: Parameters<SendVideoNoteParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendVideoNote", &params.0).await
    }

    #[tool(description = "Send paid media (photos/videos that require Stars to view)")]
    async fn send_paid_media(&self, params: Parameters<SendPaidMediaParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendPaidMedia", &params.0).await
    }

    #[tool(description = "Send a group of photos/videos as an album (2-10 items)")]
    async fn send_media_group(&self, params: Parameters<SendMediaGroupParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendMediaGroup", &params.0).await
    }

    #[tool(description = "Send a location point on the map")]
    async fn send_location(&self, params: Parameters<SendLocationParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendLocation", &params.0).await
    }

    #[tool(description = "Send a venue (location with name and address)")]
    async fn send_venue(&self, params: Parameters<SendVenueParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendVenue", &params.0).await
    }

    #[tool(description = "Send a phone contact")]
    async fn send_contact(&self, params: Parameters<SendContactParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendContact", &params.0).await
    }

    #[tool(description = "Send a native poll")]
    async fn send_poll(&self, params: Parameters<SendPollParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendPoll", &params.0).await
    }

    #[tool(description = "Send an animated emoji that shows a random value (dice, darts, basketball, etc.)")]
    async fn send_dice(&self, params: Parameters<SendDiceParams>) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendDice", &params.0).await
    }
}
