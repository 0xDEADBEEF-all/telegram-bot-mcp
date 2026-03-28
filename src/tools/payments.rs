use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::server::TelegramBotServer;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SendInvoiceParams {
    /// Chat ID (integer for user, or @username for channel)
    pub chat_id: String,
    /// Product name (1-32 characters)
    pub title: String,
    /// Product description (1-255 characters)
    pub description: String,
    /// Bot-defined invoice payload (1-128 bytes)
    pub payload: String,
    /// Currency code (ISO 4217 or "XTR" for Telegram Stars)
    pub currency: String,
    /// JSON array of LabeledPrice objects
    pub prices: serde_json::Value,
    /// Payment provider token (empty for Telegram Stars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Max tip amount in smallest currency unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// JSON array of suggested tip amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// Provider-specific data (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// True if the user's full name is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// True if the user's phone number is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// True if the user's email is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// True if the user's shipping address is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// True if the final price depends on shipping
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    /// Sends silently
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Reply parameters (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<serde_json::Value>,
    /// InlineKeyboardMarkup (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CreateInvoiceLinkParams {
    /// Product name (1-32 characters)
    pub title: String,
    /// Product description (1-255 characters)
    pub description: String,
    /// Bot-defined invoice payload (1-128 bytes)
    pub payload: String,
    /// Currency code (ISO 4217 or "XTR" for Telegram Stars)
    pub currency: String,
    /// JSON array of LabeledPrice objects
    pub prices: serde_json::Value,
    /// Payment provider token (empty for Stars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Max tip amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// Suggested tip amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Provider-specific data (JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// Photo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AnswerShippingQueryParams {
    /// Unique identifier for the shipping query
    pub shipping_query_id: String,
    /// Pass True if delivery is possible
    pub ok: bool,
    /// JSON array of ShippingOption objects (required if ok=True)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<serde_json::Value>,
    /// Error message (required if ok=False)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AnswerPreCheckoutQueryParams {
    /// Unique identifier for the pre-checkout query
    pub pre_checkout_query_id: String,
    /// Pass True if everything is alright
    pub ok: bool,
    /// Error message (required if ok=False)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetStarTransactionsParams {
    /// Number of transactions to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Max number of transactions to return (1-100, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct RefundStarPaymentParams {
    /// User ID of the user whose payment to refund
    pub user_id: i64,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EditUserStarSubscriptionParams {
    /// User ID
    pub user_id: i64,
    /// Telegram payment identifier for the subscription
    pub telegram_payment_charge_id: String,
    /// Pass True to cancel the subscription
    pub is_canceled: bool,
}

#[tool_router(router = tool_router_payments, vis = "pub")]
impl TelegramBotServer {
    #[tool(description = "Send an invoice for payment")]
    async fn send_invoice(
        &self,
        params: Parameters<SendInvoiceParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("sendInvoice", &params.0).await
    }

    #[tool(description = "Create a link for an invoice")]
    async fn create_invoice_link(
        &self,
        params: Parameters<CreateInvoiceLinkParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("createInvoiceLink", &params.0).await
    }

    #[tool(description = "Answer a shipping query (if invoice needs shipping)")]
    async fn answer_shipping_query(
        &self,
        params: Parameters<AnswerShippingQueryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("answerShippingQuery", &params.0)
            .await
    }

    #[tool(description = "Answer a pre-checkout query (confirm or reject payment)")]
    async fn answer_pre_checkout_query(
        &self,
        params: Parameters<AnswerPreCheckoutQueryParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("answerPreCheckoutQuery", &params.0)
            .await
    }

    #[tool(description = "Get the bot's Telegram Star transactions")]
    async fn get_star_transactions(
        &self,
        params: Parameters<GetStarTransactionsParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api.call_method("getStarTransactions", &params.0).await
    }

    #[tool(description = "Refund a Telegram Star payment")]
    async fn refund_star_payment(
        &self,
        params: Parameters<RefundStarPaymentParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("refundStarPayment", &params.0)
            .await
    }

    #[tool(description = "Edit a user's Star subscription (cancel/resume)")]
    async fn edit_user_star_subscription(
        &self,
        params: Parameters<EditUserStarSubscriptionParams>,
    ) -> Result<CallToolResult, McpError> {
        self.api
            .call_method_bool("editUserStarSubscription", &params.0)
            .await
    }
}
