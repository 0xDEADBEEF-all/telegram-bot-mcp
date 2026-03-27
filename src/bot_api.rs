use reqwest::Client;
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData as McpError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone)]
pub struct BotApiClient {
    token: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
}

impl BotApiClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    async fn raw_call<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<T, String> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.token, method);

        let resp = self
            .client
            .post(&url)
            .json(params)
            .send()
            .await
            .map_err(|e| format!("HTTP error: {e}"))?;

        let body: TelegramResponse<T> = resp
            .json()
            .await
            .map_err(|e| format!("Parse error: {e}"))?;

        if body.ok {
            body.result.ok_or_else(|| "Empty result".to_string())
        } else {
            Err(body
                .description
                .unwrap_or_else(|| "Unknown error".to_string()))
        }
    }

    /// Call a Telegram Bot API method, return CallToolResult with pretty JSON.
    pub async fn call_method(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<CallToolResult, McpError> {
        match self.raw_call::<Value>(method, params).await {
            Ok(v) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&v).unwrap_or_else(|_| v.to_string()),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// Call a Telegram Bot API method that returns true on success.
    pub async fn call_method_bool(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<CallToolResult, McpError> {
        match self.raw_call::<bool>(method, params).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("true")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
        }
    }

    /// Call a Telegram Bot API method, return raw JSON result (for CLI mode).
    pub async fn call_raw(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<Value, String> {
        self.raw_call::<Value>(method, params).await
    }

    /// Call a Telegram Bot API method with no parameters.
    pub async fn call_method_no_params(
        &self,
        method: &str,
    ) -> Result<CallToolResult, McpError> {
        self.call_method(method, &serde_json::json!({})).await
    }
}
