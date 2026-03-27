use std::future::Future;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ListToolsResult, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler};

use crate::bot_api::BotApiClient;

#[derive(Clone)]
pub struct TelegramBotServer {
    pub(crate) api: BotApiClient,
    tool_router: ToolRouter<Self>,
}

impl TelegramBotServer {
    pub fn new(token: String) -> Self {
        Self {
            api: BotApiClient::new(token),
            tool_router: Self::combined_tool_router(),
        }
    }
}

impl ServerHandler for TelegramBotServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(
            "Telegram Bot API MCP server. Full coverage of all Bot API methods: \
             messages, media, chats, forums, stickers, inline, payments, games, and more."
                .into(),
        );
        info
    }

    fn list_tools(
        &self,
        _: Option<rmcp::model::PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        let tools = self.tool_router.list_all();
        async move {
            Ok(ListToolsResult {
                meta: None,
                next_cursor: None,
                tools,
            })
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        let tool_context = ToolCallContext::new(self, request, context);
        self.tool_router.call(tool_context)
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.tool_router.get(name).cloned()
    }
}
