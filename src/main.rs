mod app;
mod args;
mod auth;
mod data;

use anyhow::anyhow;
use anyhow::Result;
use args::Command;

fn main() -> Result<()> {
    let initialized = data::vault_exists()?;
    let command = args::get_command();

    if command != Command::Init {
        if !initialized {
            return Err(anyhow!("Arcanum is not initialized."));
        }

        if !auth::is_authorized()? {
            return Err(anyhow!("Permission denied."));
        }
    }

    match command {
        Command::Init => {
            if initialized {
                return Err(anyhow!("Arcanum is already initialized."));
            }

            let master_password = auth::get_master_password()?;
            let master_key = auth::generate_master_key(master_password, None)?;

            data::vault_initialize(master_key)?;

            println!("Arcanum is initialized successfully.");
        }

        Command::Add => {
            let credential = data::get_credential()?;
            data::vault_add_credential(credential)?;

            println!("Added credential successfully.");
        }

        Command::Remove => {
            let credential_name = data::get_credential_name()?;
            data::vault_remove_credential(credential_name)?;

            println!("Removed credential successfully.");
        }

        Command::List => {
            let credentials = data::vault_get_credentials()?;

            if credentials.is_empty() {
                println!("There are no credentials in the vault.");
            } else {
                for credential in credentials {
                    println!("{}", "-".repeat(50));
                    println!("({}) {}", credential.name, credential.url,);
                    println!("{}", "-".repeat(50));
                }
            }
        }

        Command::Reset => {
            data::vault_reset()?;

            println!("Arcanum is resetted successfully.");
        }
    }

    Ok(())
}
