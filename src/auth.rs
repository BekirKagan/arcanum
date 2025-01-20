use crate::data::get_vault;
use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use inquire::prompt_secret;

pub struct MasterKey {
    pub salt: String,
    pub hash: String,
}

pub fn get_master_password() -> Result<String> {
    let master_password = prompt_secret("Enter master password")?;
    Ok(master_password)
}

pub fn generate_master_key(master_password: String, salt: Option<String>) -> Result<MasterKey> {
    let master_password = master_password.as_bytes();

    let salt_string = salt.unwrap_or(SaltString::generate(&mut OsRng).to_string());
    let salt_string = SaltString::from_b64(&salt_string).expect("Failed to generate salt.");
    let salt = SaltString::as_salt(&salt_string);

    let arg2 = Argon2::default();

    let hash = arg2
        .hash_password(master_password, salt)
        .expect("Failed to hash master password.")
        .to_string();

    let master_key = MasterKey {
        salt: salt.to_string(),
        hash,
    };

    Ok(master_key)
}

pub fn is_authorized() -> Result<bool> {
    let vault = get_vault()?;

    let master_password = get_master_password()?;
    let master_key = generate_master_key(master_password, Some(vault.salt))?;

    Ok(master_key.hash == vault.hash)
}
