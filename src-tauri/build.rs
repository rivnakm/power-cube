use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

const MIGRATIONS_DIR: &str = "migrations";
const MIGRATIONS_CHECKSUMS_FILE_PATH: &str = "meta/migrations_sha256sums.json";

fn main() {
    println!("cargo::rerun-if-changed={MIGRATIONS_DIR}");
    let migrations = match get_migrations_files() {
        Ok(migrations) => migrations,
        Err(err) => {
            println!("cargo::error=Failed to read migrations files");
            println!("cargo::error={err}");
            return;
        }
    };

    let mut result: HashMap<String, String> = HashMap::new();
    for migration in migrations {
        let file_name = migration.file_name().unwrap().to_string_lossy().to_string();
        let checksum = match get_file_checksum(&migration) {
            Ok(chk) => chk,
            Err(err) => {
                println!(
                    "cargo::error=Failed to generate checksum for {:?}",
                    &migration
                );
                println!("cargo::error={err}");
                return;
            }
        };

        result.insert(file_name, checksum);
    }

    let checksums_file = Path::new(MIGRATIONS_CHECKSUMS_FILE_PATH);
    std::fs::create_dir_all(checksums_file.parent().unwrap()).unwrap();
    if let Err(err) = std::fs::write(checksums_file, serde_json::to_string(&result).unwrap()) {
        println!("cargo::error=Failed to write migrations checksums to file");
        println!("cargo::error={err}");
        return;
    }

    tauri_build::build();
}

fn get_migrations_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let migrations = WalkDir::new(MIGRATIONS_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.file_name().unwrap().as_encoded_bytes().ends_with(b".sql"))
        .collect();

    Ok(migrations)
}

fn get_file_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;

    let digest = Sha256::digest(&buffer);

    Ok(hex::encode(digest))
}
