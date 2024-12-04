mod args;
use args::ArcanumCommands;

use dirs_next::{data_dir, data_local_dir};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rpassword::prompt_password;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};

const DATA_FILENAME: &str = "arcanum.data";

fn get_data_filepath() -> PathBuf {
    let data_dir = data_local_dir().unwrap_or(data_dir().expect("Failed to get data directory."));
    let data_filepath = Path::join(&data_dir, DATA_FILENAME);
    data_filepath
}

fn main() {
    let command = args::parse_command();

    match command {
        ArcanumCommands::Init => {
            let filepath = get_data_filepath();

            match File::create_new(&filepath) {
                Ok(mut file) => {
                    let password = prompt_password("Enter master password:")
                        .expect("Failed to get master password.");

                    let salt = SaltString::generate(&mut OsRng);
                    let arg2 = Argon2::default();

                    let hash_string = arg2
                        .hash_password(password.as_bytes(), &salt)
                        .expect("Failed to hash master password.")
                        .to_string();

                    let hash = PasswordHash::new(&hash_string)
                        .expect("Failed to generate hash.")
                        .to_string();

                    file.write_all(hash.as_bytes())
                        .expect("Failed to save hash.");

                    println!("Successfully initialized.");
                }

                Err(error) => {
                    if error.kind() == std::io::ErrorKind::AlreadyExists {
                        println!("Already initialized!");
                    } else {
                        println!(
                            "Failed to create data file in directory: {}",
                            filepath.to_str().unwrap()
                        );
                    }
                }
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
