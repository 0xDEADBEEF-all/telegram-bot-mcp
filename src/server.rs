use rmcp::model::ServerInfo;
use rmcp::{tool, ServerHandler};
use serde::Serialize;
use serde_json::{json, Value};

use crate::bot_api::BotApiClient;

#[derive(Clone)]
pub struct TelegramBotServer {
    api: BotApiClient,
}

/// Empty params helper
#[derive(Serialize)]
struct Empty {}

impl TelegramBotServer {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| anyhow::anyhow!("TELEGRAM_BOT_TOKEN env var is required"))?;
        Ok(Self {
            api: BotApiClient::new(token),
        })
    }
}

#[tool(tool_box)]
impl TelegramBotServer {
    // ─── Bot Info ───

    #[tool(description = "Get basic information about the bot (username, id, name)")]
    async fn get_me(&self) -> String {
        match self.api.call_raw("getMe", &Empty {}).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Messages ───

    #[tool(description = "Send a text message to a chat/channel")]
    async fn send_message(
        &self,
        #[tool(param, description = "Chat ID or @channel_username")] chat_id: String,
        #[tool(param, description = "Message text (supports HTML)")] text: String,
        #[tool(param, description = "Parse mode: HTML, Markdown, MarkdownV2 (optional)")] parse_mode: Option<String>,
    ) -> String {
        let mut params = json!({ "chat_id": chat_id, "text": text });
        if let Some(pm) = parse_mode {
            params["parse_mode"] = Value::String(pm);
        }
        match self.api.call_raw("sendMessage", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Edit a text message")]
    async fn edit_message_text(
        &self,
        #[tool(param, description = "Chat ID or @channel_username")] chat_id: String,
        #[tool(param, description = "Message ID to edit")] message_id: i64,
        #[tool(param, description = "New text")] text: String,
        #[tool(param, description = "Parse mode (optional)")] parse_mode: Option<String>,
    ) -> String {
        let mut params = json!({ "chat_id": chat_id, "message_id": message_id, "text": text });
        if let Some(pm) = parse_mode {
            params["parse_mode"] = Value::String(pm);
        }
        match self.api.call_raw("editMessageText", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Delete a message")]
    async fn delete_message(
        &self,
        #[tool(param, description = "Chat ID")] chat_id: String,
        #[tool(param, description = "Message ID")] message_id: i64,
    ) -> String {
        let params = json!({ "chat_id": chat_id, "message_id": message_id });
        match self.api.call_bool("deleteMessage", &params).await {
            Ok(_) => "Message deleted".to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Pin a message in a chat")]
    async fn pin_chat_message(
        &self,
        #[tool(param, description = "Chat ID")] chat_id: String,
        #[tool(param, description = "Message ID to pin")] message_id: i64,
    ) -> String {
        let params = json!({ "chat_id": chat_id, "message_id": message_id });
        match self.api.call_bool("pinChatMessage", &params).await {
            Ok(_) => "Message pinned".to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Forward a message from one chat to another")]
    async fn forward_message(
        &self,
        #[tool(param, description = "Target chat ID")] chat_id: String,
        #[tool(param, description = "Source chat ID")] from_chat_id: String,
        #[tool(param, description = "Message ID to forward")] message_id: i64,
    ) -> String {
        let params = json!({ "chat_id": chat_id, "from_chat_id": from_chat_id, "message_id": message_id });
        match self.api.call_raw("forwardMessage", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Chat/Channel Management ───

    #[tool(description = "Get information about a chat/channel")]
    async fn get_chat(
        &self,
        #[tool(param, description = "Chat ID or @channel_username")] chat_id: String,
    ) -> String {
        let params = json!({ "chat_id": chat_id });
        match self.api.call_raw("getChat", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get the number of members in a chat")]
    async fn get_chat_member_count(
        &self,
        #[tool(param, description = "Chat ID or @channel_username")] chat_id: String,
    ) -> String {
        let params = json!({ "chat_id": chat_id });
        match self.api.call_raw("getChatMemberCount", &params).await {
            Ok(v) => v.to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create an invite link for a chat")]
    async fn create_chat_invite_link(
        &self,
        #[tool(param, description = "Chat ID")] chat_id: String,
        #[tool(param, description = "Link name (optional)")] name: Option<String>,
        #[tool(param, description = "Max number of uses, 0 = unlimited (optional)")] member_limit: Option<i64>,
    ) -> String {
        let mut params = json!({ "chat_id": chat_id });
        if let Some(n) = name {
            params["name"] = Value::String(n);
        }
        if let Some(limit) = member_limit {
            params["member_limit"] = json!(limit);
        }
        match self.api.call_raw("createChatInviteLink", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Bot Commands & Menu ───

    #[tool(description = "Set the bot's command list shown in the menu")]
    async fn set_my_commands(
        &self,
        #[tool(param, description = "JSON array of {command, description} objects")] commands_json: String,
    ) -> String {
        let commands: Value = match serde_json::from_str(&commands_json) {
            Ok(v) => v,
            Err(e) => return format!("Invalid JSON: {e}"),
        };
        let params = json!({ "commands": commands });
        match self.api.call_bool("setMyCommands", &params).await {
            Ok(_) => "Commands updated".to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Webhook Management ───

    #[tool(description = "Set the webhook URL for receiving updates")]
    async fn set_webhook(
        &self,
        #[tool(param, description = "HTTPS URL to receive updates")] url: String,
        #[tool(param, description = "List of update types to receive, e.g. message,pre_checkout_query,callback_query (optional, comma-separated)")] allowed_updates: Option<String>,
    ) -> String {
        let mut params = json!({ "url": url });
        if let Some(updates) = allowed_updates {
            let list: Vec<&str> = updates.split(',').map(|s| s.trim()).collect();
            params["allowed_updates"] = json!(list);
        }
        match self.api.call_bool("setWebhook", &params).await {
            Ok(_) => "Webhook set".to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get current webhook status")]
    async fn get_webhook_info(&self) -> String {
        match self.api.call_raw("getWebhookInfo", &Empty {}).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Remove the webhook (switch to getUpdates mode)")]
    async fn delete_webhook(&self) -> String {
        match self.api.call_bool("deleteWebhook", &Empty {}).await {
            Ok(_) => "Webhook deleted".to_string(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Payments (Telegram Stars) ───

    #[tool(description = "Create a payment invoice link for Telegram Stars")]
    async fn create_invoice_link(
        &self,
        #[tool(param, description = "Product name")] title: String,
        #[tool(param, description = "Product description")] description: String,
        #[tool(param, description = "Bot-defined invoice payload (e.g. userId_paymentId)")] payload: String,
        #[tool(param, description = "Price in Telegram Stars")] amount: i64,
    ) -> String {
        let params = json!({
            "title": title,
            "description": description,
            "payload": payload,
            "currency": "XTR",
            "prices": [{ "label": title, "amount": amount }]
        });
        match self.api.call_raw("createInvoiceLink", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // ─── Updates (Polling/Monitoring) ───

    #[tool(description = "Get recent updates (messages, payments, etc.) — only works if webhook is not set")]
    async fn get_updates(
        &self,
        #[tool(param, description = "Max number of updates to return (1-100, default 10)")] limit: Option<i64>,
        #[tool(param, description = "Offset to start from (use update_id+1 from last update)")] offset: Option<i64>,
    ) -> String {
        let mut params = json!({});
        params["limit"] = json!(limit.unwrap_or(10));
        if let Some(off) = offset {
            params["offset"] = json!(off);
        }
        match self.api.call_raw("getUpdates", &params).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for TelegramBotServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Telegram Bot API MCP server. Manage bots, channels, messages, webhooks, and Telegram Stars payments.".into()),
            ..Default::default()
        }
    }
}
