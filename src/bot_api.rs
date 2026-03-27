use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Telegram Bot API client
#[derive(Clone)]
pub struct BotApiClient {
    token: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
}

impl BotApiClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    /// Call any Telegram Bot API method
    pub async fn call<T: serde::de::DeserializeOwned>(
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
            Err(body.description.unwrap_or_else(|| "Unknown error".to_string()))
        }
    }

    /// Call method that returns true on success
    pub async fn call_bool(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<bool, String> {
        self.call::<bool>(method, params).await
    }

    /// Call method returning raw JSON
    pub async fn call_raw(
        &self,
        method: &str,
        params: &impl Serialize,
    ) -> Result<Value, String> {
        self.call::<Value>(method, params).await
    }
}
