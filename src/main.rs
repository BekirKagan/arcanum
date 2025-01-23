mod app;
mod args;
mod auth;
mod storage;
mod vault;

use anyhow::anyhow;
use anyhow::Result;
use args::Command;
use storage::Storage;
use vault::Credential;
use vault::Vault;

fn main() -> Result<()> {
    let command = args::get_command();

    let storage = Storage::new()?;
    let initialized = storage.vault_exists()?;

    if command != Command::Init {
        if !initialized {
            return Err(anyhow!("Arcanum is not initialized."));
        }

        let vault = storage.load_vault()?;
        let master_key = vault.get_master_key()?;
        let authorized = auth::is_authorized(master_key)?;

        if !authorized {
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
            let vault = Vault::new(master_key.hash, master_key.salt);
            storage.initialize()?;
            storage.save_vault(vault)?;
            println!("Arcanum is initialized successfully.");
        }

        Command::Add => {
            let credential = Credential::prompt()?;
            let mut vault = storage.load_vault()?;
            vault.add_credential(credential)?;
            storage.save_vault(vault)?;
            println!("Added credential successfully.");
        }

        Command::Remove => {
            let name = Credential::prompt_name()?;
            let mut vault = storage.load_vault()?;
            vault.remove_credential(&name)?;
            storage.save_vault(vault)?;
            println!("Removed credential successfully.");
        }

        Command::List => {
            let vault = storage.load_vault()?;
            for credential in vault.get_credentials()? {
                println!("{credential}");
            }
        }

        Command::Reset => {
            storage.remove_vault()?;
            println!("Arcanum is resetted successfully.");
        }
    }

    Ok(())
}
