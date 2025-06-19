use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use sqlx::Executor;
use sqlx::SqliteConnection;
use std::cmp::Ordering;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

const MIGRATIONS_SHA256SUMS_JSON: &str = include_str!("../../meta/migrations_sha256sums.json");
lazy_static! {
    static ref MIGRATIONS_SHA256SUMS: HashMap<String, String> =
        serde_json::from_str(MIGRATIONS_SHA256SUMS_JSON).unwrap();
}

pub async fn apply_migrations(
    mut conn: SqliteConnection,
    migrations_dir: PathBuf,
    db_meta_path: PathBuf,
) -> Result<(), anyhow::Error> {
    let latest_migration = if db_meta_path.exists() {
        Some(std::fs::read_to_string(&db_meta_path).unwrap())
    } else {
        None
    };
    let mut migrations = WalkDir::new(migrations_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.file_name().unwrap().as_encoded_bytes().ends_with(b".sql"))
        .collect::<Vec<PathBuf>>();

    migrations.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));

    if let Some(latest_migration) = latest_migration {
        migrations.retain(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .cmp(&latest_migration)
                == Ordering::Greater
        });
    }

    for migration in migrations.iter() {
        verify_migration_checksum(migration)?;

        let sql = std::fs::read_to_string(migration)?;
        conn.execute(sqlx::raw_sql(&sql)).await?;
    }

    let last_migration = migrations
        .last()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .unwrap_or_default();
    std::fs::write(db_meta_path, last_migration)?;

    Ok(())
}

fn verify_migration_checksum(path: &Path) -> Result<(), anyhow::Error> {
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;

    let digest = hex::encode(Sha256::digest(&buffer));

    let Some(checksum) =
        MIGRATIONS_SHA256SUMS.get(&path.file_name().unwrap().to_string_lossy().to_string())
    else {
        return Err(MigrationValidationError::UnrecognizedMigration.into());
    };

    if digest != *checksum {
        Err(MigrationValidationError::ChecksumMismatch {
            expected: checksum.clone(),
            actual: digest,
        }
        .into())
    } else {
        Ok(())
    }
}

#[derive(Debug)]
enum MigrationValidationError {
    ChecksumMismatch { expected: String, actual: String },
    UnrecognizedMigration,
}

impl Display for MigrationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "Migration checksum does not match; Expected: {}, Actual: {}",
                    expected, actual
                )
            }
            Self::UnrecognizedMigration => write!(f, "Unrecognized migration file"),
        }
    }
}

impl Error for MigrationValidationError {}
