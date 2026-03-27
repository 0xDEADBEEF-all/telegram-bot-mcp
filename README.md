# telegram-bot-mcp

MCP server and CLI for the Telegram Bot API. Full coverage of **130 methods** across all API categories.

Built with Rust, [rmcp](https://github.com/modelcontextprotocol/rust-sdk) 1.3, tokio, reqwest.

## Features

- **130 Bot API methods** — messages, media, chats, forums, stickers, inline, payments, games, passport, gifts
- **MCP server** (stdio transport) — works with Claude Code, Claude Desktop, and any MCP client
- **CLI mode** — direct method calls for agents and scripts without MCP support
- **Single binary** — no runtime dependencies, fast startup

## API Coverage

| Module | Methods | Examples |
|---|---|---|
| Bot info | 16 | `getMe`, `setMyCommands`, `setMyName`, `setMyDescription` |
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
| Other | 7 | `getFile`, `getUserProfilePhotos`, `sendGift`, `getAvailableGifts` |

## Installation

```bash
cargo build --release
```

## Usage

### MCP Server (default)

```bash
# via environment variable
TELEGRAM_BOT_TOKEN=123:abc ./telegram-bot-mcp

# via flag
./telegram-bot-mcp --token 123:abc
```

Add to your MCP config (`.mcp.json`):

```json
{
  "mcpServers": {
    "telegram-bot": {
      "type": "stdio",
      "command": "/path/to/telegram-bot-mcp",
      "env": {
        "TELEGRAM_BOT_TOKEN": "your-bot-token"
      }
    }
  }
}
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
    └── other.rs     # Passport, gifts, files (7 methods)
```

## License

MIT
