mod bot;
mod business;
mod chats;
mod editing;
mod forum;
mod games;
mod inline;
mod media;
mod messages;
mod other;
mod payments;
mod stickers;
mod stories;
mod updates;

use rmcp::handler::server::tool::ToolRouter;

use crate::server::TelegramBotServer;

impl TelegramBotServer {
    pub fn combined_tool_router() -> ToolRouter<Self> {
        Self::tool_router_bot()
            + Self::tool_router_updates()
            + Self::tool_router_messages()
            + Self::tool_router_editing()
            + Self::tool_router_media()
            + Self::tool_router_chats()
            + Self::tool_router_forum()
            + Self::tool_router_stickers()
            + Self::tool_router_inline()
            + Self::tool_router_payments()
            + Self::tool_router_games()
            + Self::tool_router_other()
            + Self::tool_router_business()
            + Self::tool_router_stories()
    }
}
