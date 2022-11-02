use clap::{Args, Parser, Subcommand, ValueEnum};
use clashrsctl::config::{ConfigLogLevel, ConfigMode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'a', long = "addr")]
    /// IP address or domain name the clash controller is listening
    pub server: Option<String>,

    #[arg(short, long)]
    /// The port which the clash controller is listening
    pub port: Option<u16>,

    #[arg(short, long)]
    /// authentication secret
    pub secret: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Config(Config),
    /// List all rules
    Rules,
    Proxy(Proxy),
    /// Print realtime log.
    /// NOT supported now
    Log,
    /// Print the traffic.
    /// NOT supported now
    Traffic,
}

#[derive(Args, Debug)]
pub struct Server {
    addr: String,
}

#[derive(Args, Debug, Clone)]
pub struct Proxy{
    #[command(subcommand)]
    pub command: ProxyCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ProxyCommand {
    /// List all proxis
    List,
    /// Check and print the delay to the specified url with a proxy
    Delay {
        proxy: String,
        url: String,
        timeout: u32,
    },
    /// Check the info of a proxy
    Info {
        proxy: String,
    },
    /// Change the selected proxy of a selector
    Change {
        proxy: String,
        new_proxy: String,
    },
}

#[derive(Args, Debug, Clone)]
pub struct Config {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommand {
    /// Check and print the clash configuration
    List,
    /// Make the clash core load a configuration file with either a absolute path or a relative
    /// path
    Load {
        path: String,
    },
    /// Send a patch of the currently loaded configuration
    Patch {
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short, long)]
        socks_port: Option<u16>,
        #[arg(short, long)]
        redir_port: Option<u16>,
        #[arg(short, long)]
        tproxy_port: Option<u16>,
        #[arg(short, long)]
        mixed_port: Option<u16>,
        #[arg(short, long)]
        ipv6: Option<bool>,
        #[arg(short, long)]
        bind_address: Option<String>,
        #[arg(short, long)]
        allow_lan: Option<bool>,
        #[arg(value_enum, short, long)]
        mode: Option<ConfigMode>,
        #[arg(value_enum, short, long)]
        log_level: Option<ConfigLogLevel>,
    },
}

// #[derive(Clone, Debug, ValueEnum)]
// pub enum LogLevel {
//     Info,
//     Warning,
//     Error,
//     Debug,
// }
//
// #[derive(Debug, ValueEnum, Clone)]
// pub enum Mode {
//     Global,
//     Rule,
//     Direct,
// }
