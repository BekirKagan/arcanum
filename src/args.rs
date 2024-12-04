use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
struct ArcanumArgs {
    #[command(subcommand)]
    command: Option<ArcanumCommands>,
}

#[derive(Subcommand)]
pub enum ArcanumCommands {
    /// Initialize the password manager and create a master password.
    Init,

    /// Add a new credential.
    Add,

    ///  List all saved credentials (show only names/descriptions).
    List,

    /// Retrieve a specific credential (prompts for master password).
    Get {
        #[arg(short, long)]
        name: String,
    },

    /// Update an existing credential.
    Edit {
        #[arg(short, long)]
        name: String,
    },

    /// Delete a credential.
    Delete {
        #[arg(short, long)]
        name: String,
    },

    /// Generate a secure password.
    Generate,

    /// Export credentials to an encrypted file.
    Export {
        #[arg(short, long)]
        file: String,
    },

    /// Import credentials from an encrypted file.
    Import {
        #[arg(short, long)]
        file: String,
    },

    /// Manually lock the application.
    Lock,
}

pub fn parse_command() -> ArcanumCommands {
    let args = ArcanumArgs::parse();
    let command = args.command.unwrap();
    command
}
