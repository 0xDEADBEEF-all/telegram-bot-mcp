# telegram-bot-mcp

[![Crates.io](https://img.shields.io/crates/v/telegram-bot-mcp)](https://crates.io/crates/telegram-bot-mcp)
[![Downloads](https://img.shields.io/crates/d/telegram-bot-mcp)](https://crates.io/crates/telegram-bot-mcp)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Linux](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-linux.yml/badge.svg)](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-linux.yml)
[![Build Windows](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-windows.yml/badge.svg)](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-windows.yml)
[![Build macOS](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-macos.yml/badge.svg)](https://github.com/0xDEADBEEF-all/telegram-bot-mcp/actions/workflows/build-macos.yml)
[![Bot API](https://img.shields.io/badge/Telegram%20Bot%20API-130%20methods-26A5E4?logo=telegram)](https://core.telegram.org/bots/api)

MCP server and CLI for the Telegram Bot API. Full coverage of **166 methods** across all API categories.

Built with Rust, [rmcp](https://github.com/modelcontextprotocol/rust-sdk) 1.3, tokio, reqwest.

## Features

- **130 Bot API methods** — messages, media, chats, forums, stickers, inline, payments, games, passport, gifts
- **MCP server** (stdio transport) — works with Claude Code, Claude Desktop, and any MCP client
- **CLI mode** — direct method calls for agents and scripts without MCP support
- **Single binary** — no runtime dependencies, fast startup

## API Coverage

| Module | Methods | Examples |
|---|---|---|
| Bot info | 16 | `getMe`, `setMyCommands`, `setMyName`, `setMyProfilePhoto` |
| Updates | 4 | `getUpdates`, `setWebhook`, `deleteWebhook`, `getWebhookInfo` |
| Messages | 10 | `sendMessage`, `forwardMessage`, `copyMessage`, `pinChatMessage` |
| Editing | 9 | `editMessageText`, `editMessageCaption`, `deleteMessage` |
| Media | 14 | `sendPhoto`, `sendVideo`, `sendDocument`, `sendPoll`, `sendDice` |
| Chats | 27 | `getChat`, `banChatMember`, `promoteChatMember`, `setChatPermissions` |
| Forums | 13 | `createForumTopic`, `editForumTopic`, `closeForumTopic` |
| Stickers | 16 | `sendSticker`, `createNewStickerSet`, `setStickerSetTitle` |
| Inline | 4 | `answerCallbackQuery`, `answerInlineQuery` |
| Payments | 7 | `sendInvoice`, `createInvoiceLink`, `getStarTransactions` |
| Games | 3 | `sendGame`, `setGameScore`, `getGameHighScores` |
| Business | 11 | `readBusinessMessage`, `setBusinessAccountName`, `transferBusinessAccountStars` |
| Stories | 4 | `postStory`, `editStory`, `deleteStory`, `repostStory` |
| Other | 28 | `sendGift`, `verifyUser`, `sendChecklist`, `getUserGifts`, `getFile` |

## Installation

### From crates.io

```bash
cargo install telegram-bot-mcp
```

### From source

```bash
git clone https://github.com/0xDEADBEEF-all/telegram-bot-mcp.git
cd telegram-bot-mcp
cargo build --release
```

## Setup

1. Get a bot token from [@BotFather](https://t.me/BotFather)
2. Add to your MCP config (`~/.mcp.json` or project `.mcp.json`):

```json
{
  "mcpServers": {
    "telegram-bot": {
      "command": "telegram-bot-mcp",
      "env": {
        "TELEGRAM_BOT_TOKEN": "your-token-from-botfather"
      }
    }
  }
}
```

3. Restart your MCP client (Claude Code, Claude Desktop, etc.)

## Usage

### MCP Server (default)

```bash
# Starts stdio MCP server (used automatically by MCP clients)
TELEGRAM_BOT_TOKEN=123:abc telegram-bot-mcp

# or via flag
telegram-bot-mcp --token 123:abc
```

### CLI Mode

```bash
# List all available methods
./telegram-bot-mcp list

# Call a method (no params)
./telegram-bot-mcp -t 123:abc call getMe

# Call with JSON params
./telegram-bot-mcp -t 123:abc call sendMessage '{"chat_id":"123","text":"hello"}'

# Call with token from env
TELEGRAM_BOT_TOKEN=123:abc ./telegram-bot-mcp call sendDice '{"chat_id":"123","emoji":"🎰"}'
```

CLI output is JSON on stdout, errors on stderr, exit code 0/1.

## Project Structure

```
src/
├── main.rs          # Entry point, CLI parsing, MCP transport
├── bot_api.rs       # Telegram Bot API HTTP client
├── cli.rs           # CLI mode handler
├── server.rs        # MCP ServerHandler + ToolRouter
└── tools/
    ├── mod.rs       # Router composition
    ├── bot.rs       # Bot info & commands (16 methods)
    ├── updates.rs   # Updates & webhooks (4 methods)
    ├── messages.rs  # Sending & pinning (10 methods)
    ├── editing.rs   # Editing & deleting (9 methods)
    ├── media.rs     # Photos, videos, polls, etc. (14 methods)
    ├── chats.rs     # Chat management (27 methods)
    ├── forum.rs     # Forum topics (13 methods)
    ├── stickers.rs  # Sticker operations (16 methods)
    ├── inline.rs    # Inline & callbacks (4 methods)
    ├── payments.rs  # Invoices & Stars (7 methods)
    ├── games.rs     # Games (3 methods)
    ├── business.rs  # Business accounts (11 methods)
    ├── stories.rs   # Stories (4 methods)
    └── other.rs     # Gifts, verification, passport, etc. (28 methods)
```

## License

MIT
