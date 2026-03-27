use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

mod bot_api;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("telegram_bot_mcp=info".parse()?),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Telegram Bot MCP server starting...");

    let transport = rmcp::transport::io::stdio();
    let server = server::TelegramBotServer::from_env()?;
    let service = server.serve(transport).await?;
    service.waiting().await?;

    Ok(())
}
