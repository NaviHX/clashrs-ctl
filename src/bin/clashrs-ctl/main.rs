use clap::Parser;
use clashrsctl_core::{ClashRequestBuilder, ClashRequest};
use tokio;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    #[cfg(debug_assertions)]
    println!("{:?}", cli);

    let mut client = ClashRequestBuilder::new();
    if cli.server.is_some() { client = client.ip(&cli.server.unwrap()); }
    if cli.port.is_some() { client = client.port(cli.port.unwrap()); }

    use cli::Command;
    match cli.command {
        Command::Rules => {
            let res = client.rule().send().await?;

            // TODO: prettify output
            println!("{:?}", res);
        }
        Command::Config(cli::Config{ command }) => {
            use cli::ConfigCommand;
            let client = client.config();

            match command {
                ConfigCommand::List => {
                    let res = client.get().send().await?;

                    // TODO: prettify output
                    println!("{:?}", res);
                }
                ConfigCommand::Load { path } => {
                    client.load(&path).send().await?
                }
                ConfigCommand::Patch { port, socks_port, redir_port, allow_lan, mode, log_level } => {
                    let mut client = client.patch();

                    if let Some(port) = port { client = client.port(port) }
                    if let Some(socks_port) = socks_port { client = client.socks_port(socks_port) }
                    if let Some(redir_port) = redir_port { client = client.redir_port(redir_port) }
                    if let Some(allow_lan) = allow_lan { client = client.allow_lan(allow_lan) }
                    if let Some(mode) = mode { client = client.mode(mode) }
                    if let Some(level) = log_level { client = client.log_level(level) }

                    client.send().await?
                }
            }
        }
        _ => {
            println!("not supported now");
        }
    }

    Ok(())
}

