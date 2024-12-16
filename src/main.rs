mod args;
use args::ArcanumCommands;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use dirs_next::data_local_dir;

use dialoguer;
use rpassword::prompt_password;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::SaltString;
use argon2::Argon2;

use serde_json::json;
use serde_json::Value;

const DATA_FILENAME: &str = "arcanum.json";
const DATA_VERSION: &str = "0.1.0";

// FIX: Error handling must be fixed.

fn get_data_filepath() -> PathBuf {
    let directory = data_local_dir().expect("Failed to find the data directory.");
    let filepath = Path::join(directory.as_path(), DATA_FILENAME);
    filepath
}

fn atomic_write_all(path: PathBuf, data: &[u8]) {
    let mut temp = File::create(path.with_extension("tmp"))
        .expect("Failed to create the temporary data file.");

    temp.write_all(data)
        .expect("Failed to write to the temporary data file.");

    temp.sync_all()
        .expect("Failed to sync the temporary data file.");

    std::fs::rename(path.with_extension("tmp"), path)
        .expect("Failed to rename the temporary data file.");
}

fn should_initialize() -> bool {
    let filepath = get_data_filepath();
    let should_initialize = filepath.try_exists().unwrap_or_default();
    !should_initialize
}

fn authenticate_user() -> bool {
    let input_password =
        prompt_password("Enter master password:").expect("Failed to retrieve master password.");

    let filepath = get_data_filepath();
    let data = std::fs::read(filepath).expect("Failed to read the data file.");
    let data: Value = serde_json::from_slice(&data).unwrap();

    let salt_string = data
        .get("salt")
        .expect("Failed to get the salt from the data file.")
        .as_str()
        .unwrap();
    let salt = SaltString::from_b64(salt_string).unwrap();

    let arg2 = Argon2::default();

    let input_hash = arg2
        .hash_password(input_password.as_bytes(), &salt)
        .expect("Failed to hash master password.")
        .to_string();

    let stored_hash = data
        .get("hash")
        .expect("Failed to get the salt from the data file.")
        .as_str()
        .unwrap();

    let is_authenticated = input_hash == stored_hash;
    is_authenticated
}

fn initialize_vault() {
    let password =
        prompt_password("Enter master password:").expect("Failed to retrieve master password.");

    let salt = SaltString::generate(&mut OsRng);

    let arg2 = Argon2::default();
    let hash = arg2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash master password.");

    let data = json!({
        "version": DATA_VERSION,
        "salt": salt.as_str(),
        "hash": hash.to_string(),
        "vault": {},
    });

    atomic_write_all(get_data_filepath(), data.to_string().as_bytes());

    println!("Initialization successful.");
}

fn reset_vault() {
    let filepath = get_data_filepath();
    std::fs::remove_file(filepath).expect("Failed to reset.");

    println!("Reset successful.");
}

fn add_credential(name: String) {
    let filepath = get_data_filepath();
    let data = std::fs::read(&filepath).expect("Failed to read the data file.");
    let mut data: Value = serde_json::from_slice(&data).unwrap();

    if let Some(vault) = data.get_mut("vault").and_then(|v| v.as_object_mut()) {
        let value_exists = vault.keys().find(|k| **k == name).is_some();

        if value_exists {
            let should_overwrite = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "There is another credential named: {name}. Would you like to overwrite it?",
                ))
                .interact()
                .expect("Failed to confirm.");

            if should_overwrite {
                let credential = prompt_credential();
                // TODO: Encrypt the credential.
                vault.insert(name, credential);
                println!("Overwritten the credential succesfuly.");
            } else {
                println!("Failed to overwrite the credential.");
            }
        } else {
            let credential = prompt_credential();
            // TODO: Encrypt the credential.
            vault.insert(name, credential);
            println!("Added the credential successfully.");
        }
    }

    atomic_write_all(filepath, data.to_string().as_bytes());
}

fn prompt_credential() -> Value {
    let username: String = dialoguer::Input::new()
        .with_prompt("Enter username")
        .interact_text()
        .expect("Failed to retrieve username.");

    let password: String = dialoguer::Password::new()
        .with_prompt("Enter password")
        .interact()
        .expect("Failed to retrieve password.");

    let notes: String = dialoguer::Input::new()
        .with_prompt("Enter notes (optional)")
        .allow_empty(true)
        .interact_text()
        .expect("Failed to retrieve notes.");

    let credential = json!({
        "username": username,
        "password": password,
        "notes": notes,
    });
    credential
}

fn main() {
    let command = args::parse_command();

    #[allow(unused_variables)]
    match command {
        ArcanumCommands::Init => {
            if should_initialize() {
                initialize_vault();
            } else {
                println!("Already initialized.");
            }
        }

        ArcanumCommands::Add { name } => {
            if !authenticate_user() {
                println!("Failed to authenticate user.");
                return;
            }

            // TODO: Check for duplicates

            add_credential(name);
        }

        ArcanumCommands::List => {}

        ArcanumCommands::Get { name } => {}

        ArcanumCommands::Edit { name } => {}

        ArcanumCommands::Delete { name } => {}

        ArcanumCommands::Generate => {}

        ArcanumCommands::Export { file } => {}

        ArcanumCommands::Import { file } => {}

        ArcanumCommands::Lock => {}

        ArcanumCommands::Reset => {
            if should_initialize() {
                println!("Failed to reset as the vault has not been initialized.");
                return;
            }

            if !authenticate_user() {
                println!("Failed to authenticate user.");
                return;
            }

            reset_vault();
        }
    }
}
