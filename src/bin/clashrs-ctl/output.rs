use std::borrow::Borrow;

use clashrsctl::{
    config::{Config, ConfigLogLevel, ConfigMode},
    proxy::{ProxyInfo, ProxyList},
    rule::{Rule, RuleList},
    stream::log::Log,
    stream::traffic::Traffic, connection::{ConnectionVec, Connection},
};

pub trait CliOutput {
    fn print(&self);
}

impl CliOutput for Rule {
    fn print(&self) {
        println!("{},\t{},\t{}", self.r#type, self.payload, self.proxy);
    }
}

impl CliOutput for RuleList {
    fn print(&self) {
        println!("TYPE,\tPAYLOAD,\tPROXY");
        for rule in self.iter() {
            rule.print();
        }
    }
}

impl CliOutput for Config {
    fn print(&self) {
        println!("HTTP port: {}", self.port.as_ref().unwrap_or_else(|| &0));
        println!(
            "SOCKS port: {}",
            self.socks_port.as_ref().unwrap_or_else(|| &0)
        );
        println!(
            "REDIR port: {}",
            self.redir_port.as_ref().unwrap_or_else(|| &0)
        );
        println!(
            "TPROXY port: {}",
            self.tproxy_port.as_ref().unwrap_or_else(|| &0)
        );
        println!(
            "MIXED port: {}",
            self.mixed_port.as_ref().unwrap_or_else(|| &0)
        );
        println!(
            "Allow LAN: {}",
            self.allow_lan.as_ref().unwrap_or_else(|| &false)
        );
        println!("IPv6: {}", self.ipv6.as_ref().unwrap_or_else(|| &false));
        println!(
            "Bind Address: {}",
            self.bind_address
                .as_ref()
                .map(|s| s.borrow())
                .unwrap_or_else(|| "*")
        );
        println!(
            "Mode: {}",
            self.mode
                .as_ref()
                .map(|mode| match mode {
                    ConfigMode::Global => "Global",
                    ConfigMode::Rule => "Rule",
                    ConfigMode::Direct => "Direct",
                })
                .unwrap_or_else(|| "None")
        );
        println!(
            "Log level: {}",
            self.log_level
                .as_ref()
                .map(|level| match level {
                    ConfigLogLevel::Info => "Info",
                    ConfigLogLevel::Warning => "Warning",
                    ConfigLogLevel::Error => "Error",
                    ConfigLogLevel::Debug => "Debug",
                })
                .unwrap_or_else(|| "None")
        );
    }
}

impl CliOutput for ProxyInfo {
    fn print(&self) {
        if let Some(t) = self.r#type.as_ref() {
            println!("Type: {}", t);
        }

        if let Some(list) = self.all.as_ref() {
            println!("Proxy:");
            for item in list.iter() {
                println!("- {}", item);
            }
        }

        if let Some(cur) = self.now.as_ref() {
            println!("Selected: {}", cur);
        }
    }
}

impl CliOutput for ProxyList {
    fn print(&self) {
        for (name, map) in self.iter() {
            println!("{}: {}", name, map["type"]);
        }
    }
}

impl CliOutput for Traffic {
    fn print(&self) {
        println!("up:{}, down:{}", self.up, self.down);
    }
}

impl CliOutput for Log {
    fn print(&self) {
        println!("[{}]: {}",
                 match self.r#type{
                     ConfigLogLevel::Info => "INFO",
                     ConfigLogLevel::Warning => "WARN",
                     ConfigLogLevel::Error => "ERROR",
                     ConfigLogLevel::Debug => "DEBUG"
                 },
                 self.payload);
    }
}

impl CliOutput for ConnectionVec {
    fn print(&self) {
        println!("Total upload: {}", self.upload_total);
        println!("Total download: {}", self.download_total);

        println!("ID\tType");
        for connection in self.connections.iter() {
            <Connection as CliOutput>::print(&connection);
        }
    }
}

impl CliOutput for Connection {
    fn print(&self) {
        println!("{}\t{}", self.id, self.metadata.r#type);
    }
}
