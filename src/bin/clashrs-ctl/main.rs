use clap::Parser;
use clashrsctl::{ClashRequestBuilder, ClashRequest};
use tokio;
use crate::output::CliOutput;
use futures::StreamExt;

mod cli;
mod output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    #[cfg(debug_assertions)]
    println!("{:?}", cli);

    let mut client = ClashRequestBuilder::new();
    if cli.server.is_some() { client = client.ip(&cli.server.unwrap()); }
    if cli.port.is_some() { client = client.port(cli.port.unwrap()); }
    if cli.secret.is_some() { client = client.secret(&cli.secret.unwrap()); }

    use cli::Command;
    match cli.command {
        Command::Rules => {
            let res = client.rule().send().await?;

            // TODO: prettify output
            // println!("{:?}", res);
            res.print();
        }
        Command::Config(cli::Config{ command }) => {
            use cli::ConfigCommand;
            let client = client.config();

            match command {
                ConfigCommand::List => {
                    let res = client.get().send().await?;

                    // TODO: prettify output
                    // println!("{:?}", res);
                    res.print()
                }
                ConfigCommand::Load { path } => {
                    client.load(&path).send().await?
                }
                ConfigCommand::Patch { port, socks_port, redir_port, tproxy_port, mixed_port, ipv6, bind_address, allow_lan, mode, log_level } => {
                    let mut client = client.patch();

                    if let Some(port) = port { client = client.port(port) }
                    if let Some(socks_port) = socks_port { client = client.socks_port(socks_port) }
                    if let Some(redir_port) = redir_port { client = client.redir_port(redir_port) }
                    if let Some(tproxy_port) = tproxy_port { client = client.tproxy_port(tproxy_port) }
                    if let Some(mixed_port) = mixed_port { client = client.mixed_port(mixed_port) }
                    if let Some(ipv6) = ipv6 { client = client.ipv6(ipv6) }
                    if let Some(address) = bind_address { client = client.bind_address(&address) }
                    if let Some(allow_lan) = allow_lan { client = client.allow_lan(allow_lan) }
                    if let Some(mode) = mode { client = client.mode(mode) }
                    if let Some(level) = log_level { client = client.log_level(level) }

                    client.send().await?
                }
            }
        }
        Command::Proxy(cli::Proxy{ command }) => {
            use cli::ProxyCommand;

            let client = client.proxies();
            match command {
                ProxyCommand::List => {
                    let res = client.send().await?;

                    // TODO: prettify output
                    // println!("{:?}", res);
                    res.print();
                }
                ProxyCommand::Info { proxy } => {
                    let res = client.get(&proxy).send().await?;

                    // TODO: prettify output
                    // println!("{:?}", res);
                    res.print()
                }
                ProxyCommand::Delay { proxy, url, timeout } => {
                    let clashrsctl::proxy::ProxyDelay { delay } = client.get(&proxy).delay(&url, timeout).send().await?;

                    println!("{} ms", delay);
                }
                ProxyCommand::Change { proxy, new_proxy } => {
                    client.get(&proxy).change(&new_proxy).send().await?;
                }
            }
        }
        Command::Version => {
            let version = client.version()
                .send()
                .await?;

            println!("{}", version.version);
        }
        Command::Traffic => {
            let mut traffic_stream = client.traffic().send().await?;

            while let Some(res) = traffic_stream.next().await {
                res?.print();
            }

            println!("Disconnected");
        }
        Command::Log => {
            let mut log_stream = client.logs().send().await?;

            while let Some(res) = log_stream.next().await {
                res?.print();
            }

            println!("Disconnected");
        }
        Command::Connection(cli::Connection { command}) => {
            use cli::ConnectionCommand;
            match command {
                ConnectionCommand::List => client.connections().send().await?.print(),
                ConnectionCommand::CloseAll => client.connections().close().send().await?,
                ConnectionCommand::Close { id } => client.connections().close_id(&id).send().await?,
            }
        }
    }

    Ok(())
}

