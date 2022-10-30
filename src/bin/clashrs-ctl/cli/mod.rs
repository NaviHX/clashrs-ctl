use clap::{Args, Parser, Subcommand, ValueEnum};
use clashrsctl_core::config::{ConfigLogLevel, ConfigMode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub server: Option<String>,

    #[arg(short, long)]
    pub port: Option<u16>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Config(Config),
    Rules,
    Proxy,
    Log,
    Traffic,
}

#[derive(Args, Debug)]
pub struct Server {
    addr: String,
}

#[derive(Args, Debug, Clone)]
pub struct Config {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommand {
    List,
    Load {
        path: String,
    },
    Patch {
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short, long)]
        socks_port: Option<u16>,
        #[arg(short, long)]
        redir_port: Option<u16>,
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
