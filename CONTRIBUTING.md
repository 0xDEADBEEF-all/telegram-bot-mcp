# Contributing

Thanks for your interest in contributing to telegram-bot-mcp!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/telegram-bot-mcp.git`
3. Create a branch: `git checkout -b my-feature`
4. Make your changes
5. Push and open a PR against `master`

## Development

### Prerequisites

- Rust stable (latest)
- A Telegram Bot token from [@BotFather](https://t.me/BotFather)

### Build

```bash
cargo build --release
```

### Test locally

```bash
# CLI mode
export TELEGRAM_BOT_TOKEN="your-token"
./target/release/telegram-bot-mcp call getMe

# MCP mode (stdio)
./target/release/telegram-bot-mcp
```

### Lint

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## Adding new Bot API methods

1. Find the right module in `src/tools/` (e.g., `messages.rs`, `chats.rs`)
2. Add a params struct with `#[derive(Deserialize, Serialize, JsonSchema)]`
3. Add doc comments on each field (they become the tool's parameter descriptions)
4. Add the tool method inside the `#[tool_router]` impl block
5. Run `cargo build` and `cargo clippy`

Example:

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MyNewMethodParams {
    /// Chat ID or @username
    pub chat_id: String,
    /// Optional parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}

// Inside the #[tool_router] impl block:
#[tool(description = "Description of what this method does")]
async fn my_new_method(
    &self,
    params: Parameters<MyNewMethodParams>,
) -> Result<CallToolResult, McpError> {
    self.api.call_method("myNewMethod", &params.0).await
}
```

## Code Style

- Run `cargo fmt` before committing
- No warnings from `cargo clippy`
- Keep tool methods minimal — delegate to `bot_api.rs` helpers
- Use `serde_json::Value` for complex nested Telegram types (inline keyboards, etc.)

## Pull Requests

- One feature/fix per PR
- Reference related issues (`Fixes #123`)
- PRs require passing CI checks (fmt, clippy, build on all platforms)
- PRs require one approving review
