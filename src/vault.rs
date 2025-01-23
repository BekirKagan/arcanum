use crate::auth::MasterKey;
use anyhow::anyhow;
use anyhow::Result;
use inquire::prompt_secret;
use inquire::prompt_text;
use serde::Deserialize;
use serde::Serialize;

const ARCANUM_VAULT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
pub struct Vault {
    version: String,
    hash: String,
    salt: String,
    credentials: Vec<Credential>,
}

impl Vault {
    pub fn new(hash: String, salt: String) -> Self {
        Self {
            version: ARCANUM_VAULT_VERSION.to_owned(),
            hash,
            salt,
            credentials: Vec::default(),
        }
    }

    pub fn add_credential(&mut self, credential: Credential) -> Result<()> {
        if self.credential_exists(&credential.name) {
            return Err(anyhow!("Credential already exists."));
        }
        self.credentials.push(credential);
        Ok(())
    }

    pub fn remove_credential(&mut self, name: &str) -> Result<()> {
        if !self.credential_exists(name) {
            return Err(anyhow!("Credential does not exist."));
        }
        self.credentials.retain(|c| c.name != name);
        Ok(())
    }

    pub fn get_credentials(&self) -> Result<Vec<Credential>> {
        Ok(self.credentials.clone())
    }

    pub fn get_master_key(&self) -> Result<MasterKey> {
        Ok(MasterKey {
            hash: self.hash.clone(),
            salt: self.salt.clone(),
        })
    }

    fn credential_exists(&mut self, name: &str) -> bool {
        let exists = self.credentials.iter().find(|c| c.name == name).is_some();
        exists
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Credential {
    url: String,
    name: String,
    password: String,
}

impl Credential {
    pub fn prompt() -> Result<Self> {
        let url = prompt_text("Enter credential URL")?;
        let name = prompt_text("Enter credential name")?;
        let password = prompt_secret("Enter password")?;
        Ok(Self {
            url,
            name,
            password,
        })
    }

    pub fn prompt_name() -> Result<String> {
        let name = prompt_text("Enter credential name")?;
        Ok(name)
    }
}

impl std::fmt::Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.url,)
    }
}
