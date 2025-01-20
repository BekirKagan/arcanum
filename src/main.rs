mod app;
mod args;
mod auth;
mod data;

use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let initialized = data::vault_exists()?;
    let command = args::get_command();

    match command {
        args::Command::Init => {
            if initialized {
                return Err(anyhow!("Arcanum is already initialized."));
            }

            let master_password = auth::get_master_password()?;
            let master_key = auth::generate_master_key(master_password, None)?;

            data::vault_initialize(master_key)?;

            println!("Arcanum is initialized successfully.");
        }

        args::Command::Add => {
            if !initialized {
                return Err(anyhow!("Arcanum is not initialized."));
            }

            if !auth::is_authorized()? {
                return Err(anyhow!("Permission denied."));
            }

            let credential = data::get_credential()?;
            data::vault_add_credential(credential)?;
        }

        args::Command::Remove => {
            if !initialized {
                return Err(anyhow!("Arcanum is not initialized."));
            }

            if !auth::is_authorized()? {
                return Err(anyhow!("Permission denied."));
            }

            let credential_name = data::get_credential_name()?;
            data::vault_remove_credential(credential_name)?;
        }

        args::Command::Reset => {
            if !initialized {
                return Err(anyhow!("Arcanum is not initialized."));
            }

            if !auth::is_authorized()? {
                return Err(anyhow!("Permission denied."));
            }

            data::vault_reset()?;

            println!("Arcanum is resetted successfully.");
        }
    }

    Ok(())
}
