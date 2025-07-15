use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "wt-cli",
    about,
    long_about = "Brian's custom CLI shortcuts",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show available commands
    List,
    /// Run a shortcut command (e.g., `wt notes`)
    #[command(external_subcommand)]
    Run(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_list_command() {
        let cli = Cli::parse_from(["wt", "list"]);
        assert!(matches!(cli.command, Some(Commands::List)));
    }

    #[test]
    fn parses_run_command_with_args() {
        let cli = Cli::parse_from(["wt", "notes"]);
        match cli.command {
            Some(Commands::Run(args)) => {
                assert_eq!(args, vec!["notes"]);
            }
            _ => panic!("Expected Run variant"),
        }
    }

    #[test]
    fn parses_run_command_with_multiple_args() {
        let cli = Cli::parse_from(["wt", "deploy", "prod"]);
        match cli.command {
            Some(Commands::Run(args)) => {
                assert_eq!(args, vec!["deploy", "prod"]);
            }
            _ => panic!("Expected Run variant"),
        }
    }

    #[test]
    fn parses_no_command() {
        let cli = Cli::parse_from(["wt"]);
        assert!(cli.command.is_none());
    }
}
