mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use dirs::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, fs, process::Command};

#[derive(Debug, Deserialize)]
struct Config {
    aliases: HashMap<String, String>,
}

fn load_aliases() -> HashMap<String, String> {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(".config/wt-cli/aliases.toml");
    let content = fs::read_to_string(path).expect("Failed to read aliases.toml");
    let config: Config = toml::from_str(&content).expect("Failed to parse aliases.toml");
    config.aliases
}

fn main() {
    let cli = Cli::parse();
    let aliases = load_aliases();

    match cli.command {
        Some(Commands::List) => {
            println!("📦 Brian's commands:");
            for key in aliases.keys() {
                println!("  - {}", key);
            }
        }
        Some(Commands::Run(args)) => {
            if let Some(cmd_key) = args.first() {
                if let Some(cmd) = aliases.get(cmd_key) {
                    let status = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .status()
                        .expect("Failed to run command");
                    std::process::exit(status.code().unwrap_or(1));
                } else {
                    eprintln!("⚠️ Unknown command: '{}'", cmd_key);
                    std::process::exit(1);
                }
            } else {
                eprintln!("⚠️ No command provided. Use `wt list` to see available shortcuts.");
                std::process::exit(1);
            }
        }
        None => {
            println!("ℹ️ Use `wt list` to view your commands.");
        }
    }
}
