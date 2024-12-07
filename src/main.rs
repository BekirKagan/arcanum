mod args;
use args::ArcanumCommands;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use dirs_next::data_local_dir;

use rpassword::prompt_password;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::SaltString;
use argon2::Argon2;

use serde_json::json;

const DATA_FILENAME: &str = "arcanum.json";

fn get_data_filepath() -> PathBuf {
    let directory = data_local_dir().expect("Could not find data directory.");
    let filepath = Path::join(directory.as_path(), DATA_FILENAME);
    filepath
}

fn should_initialize() -> bool {
    let filepath = get_data_filepath();
    let should_initialize = filepath.try_exists().unwrap_or_default();
    !should_initialize
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

fn initialize_vault() {
    let password =
        prompt_password("Enter master password:").expect("Failed to retrieve master password.");

    let arg2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);
    let hash = arg2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash master password.");

    let data = json!({
        "version": "0.1.0",
        "salt": salt.as_str(),
        "hash": hash.to_string(),
    });

    atomic_write_all(get_data_filepath(), data.to_string().as_bytes());

    println!("Initialization successful.");
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

        ArcanumCommands::Add => {}

        ArcanumCommands::List => {}

        ArcanumCommands::Get { name } => {}

        ArcanumCommands::Edit { name } => {}

        ArcanumCommands::Delete { name } => {}

        ArcanumCommands::Generate => {}

        ArcanumCommands::Export { file } => {}

        ArcanumCommands::Import { file } => {}

        ArcanumCommands::Lock => {}
    }
}
