use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize the password manager and create a master password.
    Init,

    /// Add credential.
    Add,

    /// Remove credential.
    Remove,

    /// Reset the password manager and delete the vault irreversibly.
    Reset,
}

pub fn get_command() -> Command {
    let args = Args::parse();
    args.command
}
