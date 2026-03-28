use clap::{Parser, Subcommand};
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

mod bot_api;
mod cli;
mod server;
mod tools;

#[derive(Parser)]
#[command(
    name = "telegram-bot-mcp",
    about = "Telegram Bot API — MCP server & CLI"
)]
struct App {
    /// Bot token (overrides TELEGRAM_BOT_TOKEN env var)
    #[arg(short, long, global = true, env = "TELEGRAM_BOT_TOKEN")]
    token: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Call a Telegram Bot API method directly
    Call {
        /// API method name (e.g. sendMessage, getMe)
        method: String,
        /// JSON parameters (optional)
        params: Option<String>,
    },
    /// List all available Telegram Bot API methods
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::parse();

    let resolve_token = || -> anyhow::Result<String> {
        app.token.clone().ok_or_else(|| {
            anyhow::anyhow!("Bot token required: use --token or set TELEGRAM_BOT_TOKEN")
        })
    };

    match app.command {
        Some(Command::Call { method, params }) => {
            let token = resolve_token()?;
            cli::call_method(&token, &method, params.as_deref()).await
        }
        Some(Command::List) => {
            cli::list_methods();
            Ok(())
        }
        None => {
            let token = resolve_token()?;

            tracing_subscriber::fmt()
                .with_env_filter(
                    EnvFilter::from_default_env().add_directive("telegram_bot_mcp=info".parse()?),
                )
                .with_writer(std::io::stderr)
                .init();

            tracing::info!("Telegram Bot MCP server starting...");

            let transport = rmcp::transport::io::stdio();
            let server = server::TelegramBotServer::new(token);
            let service = server.serve(transport).await?;
            service.waiting().await?;

            Ok(())
        }
    }
}
