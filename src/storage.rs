use crate::vault::Vault;
use anyhow::Context;
use anyhow::Result;
use dirs_next::data_local_dir;
use serde_json::json;
use std::fs::File;
use std::path::PathBuf;

const ARCANUM_VAULT_NAME: &str = env!("CARGO_PKG_NAME");

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let data_dir = data_local_dir().context("Data directory does not exist.")?;
        let path = data_dir
            .join(ARCANUM_VAULT_NAME)
            .join(ARCANUM_VAULT_NAME)
            .with_extension("json");
        Ok(Self { path })
    }

    pub fn initialize(&self) -> Result<()> {
        std::fs::create_dir(self.path.parent().unwrap())?;
        File::create_new(&self.path)?;
        Ok(())
    }

    pub fn vault_exists(&self) -> Result<bool> {
        let exists = std::fs::exists(&self.path)?;
        Ok(exists)
    }

    pub fn save_vault(&self, vault: Vault) -> Result<()> {
        let vault_json = json!(vault);
        std::fs::write(&self.path, vault_json.to_string())?;
        Ok(())
    }

    pub fn load_vault(&self) -> Result<Vault> {
        let vault_string = std::fs::read_to_string(&self.path)?;
        let vault: Vault = serde_json::from_str(vault_string.as_str())?;
        Ok(vault)
    }

    pub fn remove_vault(&self) -> Result<()> {
        std::fs::remove_file(&self.path)?;
        std::fs::remove_dir(self.path.parent().unwrap())?;
        Ok(())
    }
}
