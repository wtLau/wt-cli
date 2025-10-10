mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use dirs::home_dir;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug, Deserialize)]
struct Config {
    aliases: HashMap<String, String>,
}

// Load aliases from the TOML file
// The file is expected to be at ~/dotfile/cli-aliases.toml
// and have the following format:
// [aliases]
// notes = "nvim ~/Documents/notes"md
// doc = "cd ~/Documents"
fn load_aliases() -> HashMap<String, String> {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push("dotfile/cli-aliases.toml");
    let content = fs::read_to_string(path).expect("Failed to read aliases.toml");
    let config: Config = toml::from_str(&content).expect("Failed to parse aliases.toml");
    config.aliases
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }
    PathBuf::from(path)
}

fn main() {
    let cli = Cli::parse();
    let aliases = load_aliases();

    match cli.command {
        // Handle the List command
        Some(Commands::List) => {
            println!("📦 Brian's commands:");
            for key in aliases.keys() {
                println!("  - {}", key);
            }
        }
        // Handle cd like commands
        // e.g., wt cd doc -> cd ~/Documents
        Some(Commands::Cd { dir }) => {
            let aliases = load_aliases(); // assuming this returns Config.aliases
            if let Some(path) = aliases.get(&dir) {
                let dir_path = expand_tilde(path);
                // Spawn a new Zsh subshell in the specified directory
                let status = Command::new("zsh") // Start the "zsh" executable
                    .current_dir(&dir_path) // Set the working directory of the new shell
                    .stdin(Stdio::inherit()) // Connect the subshell's stdin to the parent terminal
                    .stdout(Stdio::inherit()) // Connect the subshell's stdout to the parent terminal
                    .stderr(Stdio::inherit()) // Connect the subshell's stderr to the parent terminal
                    .status() // Wait for the subshell to exit and capture its exit status
                    .expect("Failed to open zsh subshell");
                std::process::exit(status.code().unwrap_or(1));
            } else {
                eprintln!("⚠️ Unknown directory alias: '{}'", dir);
                std::process::exit(1);
            }
        }
        // Handle the Run command
        // e.g., wt notes -> runs the command associated with "notes"
        Some(Commands::Run(args)) => {
            if let Some(dir_key) = args.first() {
                if let Some(dir) = aliases.get(dir_key) {
                    let directory = expand_tilde(dir);
                    let command_str = format!("cd {:?} && nvim", &directory);
                    let status = Command::new("sh")
                        .arg("-c")
                        .arg(command_str)
                        .status()
                        .expect("Failed to run command");
                    std::process::exit(status.code().unwrap_or(1));
                } else {
                    eprintln!("⚠️ Unknown command: '{}'", dir_key);
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
