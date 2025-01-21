use crate::auth::MasterKey;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use dirs_next::data_local_dir;
use inquire::prompt_secret;
use inquire::prompt_text;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

const ARCANUM_VAULT_FOLDER_NAME: &str = "Arcanum";
const ARCANUM_VAULT_FILE_NAME: &str = "arcanum.json";
const ARCANUM_VAULT_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub version: String,
    pub salt: String,
    pub hash: String,
    pub credentials: Vec<Credential>,
}

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub url: String,
    pub name: String,
    pub password: String,
}

pub fn vault_exists() -> Result<bool> {
    let vault_path = get_vault_path()?;

    vault_path
        .try_exists()
        .context("Vault file could not be verified.")
}

pub fn vault_initialize(master_key: MasterKey) -> Result<()> {
    let vault_path = get_vault_path()?;
    let vault_path_parent = vault_path.parent().unwrap();

    let vault_json = json!(Vault {
        version: ARCANUM_VAULT_VERSION.unwrap_or_default().to_string(),
        salt: master_key.salt,
        hash: master_key.hash,
        credentials: Vec::<Credential>::new(),
    });

    fs::create_dir(vault_path_parent)?;
    fs::File::create_new(&vault_path)?;
    fs::write(vault_path, vault_json.to_string())?;

    Ok(())
}

pub fn get_vault() -> Result<Vault> {
    let vault_path = get_vault_path()?;

    let data = fs::read_to_string(vault_path)?;
    let vault: Vault = serde_json::from_str(&data)?;

    Ok(vault)
}

pub fn get_credential() -> Result<Credential> {
    let url = prompt_text("Enter credential URL")?;
    let name = prompt_text("Enter credential name")?;
    let password = prompt_secret("Enter password")?;

    let credential = Credential {
        url,
        name,
        password,
    };

    Ok(credential)
}

pub fn get_credential_name() -> Result<String> {
    let name = prompt_text("Enter credential name")?;

    Ok(name)
}

pub fn vault_add_credential(credential: Credential) -> Result<()> {
    let vault_path = get_vault_path()?;
    let mut vault = get_vault()?;

    if let Some(_) = vault.credentials.iter().find(|c| c.name == credential.name) {
        return Err(anyhow!("Credential: '{}' already exists.", credential.name));
    }

    vault.credentials.push(credential);

    let vault_json = json!(vault);
    fs::write(vault_path, vault_json.to_string())?;

    Ok(())
}

pub fn vault_remove_credential(credential_name: String) -> Result<()> {
    let vault_path = get_vault_path()?;
    let mut vault = get_vault()?;

    if let None = vault.credentials.iter().find(|c| c.name == credential_name) {
        return Err(anyhow!("Credential: '{}' does not exist.", credential_name));
    }

    vault.credentials.retain(|c| c.name != credential_name);

    let vault_json = json!(vault);
    fs::write(vault_path, vault_json.to_string())?;

    Ok(())
}

pub fn vault_get_credentials() -> Result<Vec<Credential>> {
    let vault = get_vault()?;

    Ok(vault.credentials)
}

pub fn vault_reset() -> Result<()> {
    let vault_path = get_vault_path()?;
    let vault_path_parent = vault_path.parent().unwrap();

    fs::remove_file(&vault_path)?;
    fs::remove_dir(vault_path_parent)?;

    Ok(())
}

fn get_vault_path() -> Result<PathBuf> {
    let data_dir = data_local_dir().context("Data directory could not be verified.")?;

    let vault_path = data_dir
        .join(ARCANUM_VAULT_FOLDER_NAME)
        .join(ARCANUM_VAULT_FILE_NAME);

    Ok(vault_path)
}
