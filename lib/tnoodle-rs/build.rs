use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use reqwest::Url;
use serde::Deserialize;

const MAVEN_CENTRAL_BASE_URL: &str = "https://repo.maven.apache.org/maven2/";
const MAVEN_DEPS_FILE_PATH: &str = "maven-deps.json";

#[derive(Deserialize)]
struct MavenDependenciesFile {
    artifacts: Vec<String>,
}

struct MavenArtifact {
    group: String,
    id: String,
    version: String,
    qualifier: String,
}

fn main() {
    let jassets_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join("../../../jassets");
    std::fs::create_dir_all(&jassets_dir).unwrap();

    println!("cargo::rerun-if-changed={MAVEN_DEPS_FILE_PATH}");

    let deps_file = PathBuf::from_str(MAVEN_DEPS_FILE_PATH).unwrap();
    let mut deps = match read_dependency_list(&deps_file) {
        Ok(deps) => deps,
        Err(err) => {
            println!("cargo::error=Failed to read dependencies file");
            println!("cargo::error={err}");
            return;
        }
    };

    // j4rs core library
    deps.push(MavenArtifact {
        group: "io.github.astonbitecode".into(),
        id: "j4rs".into(),
        version: "0.22.0".into(),
        qualifier: "-jar-with-dependencies".into(),
    });

    // Library dependencies
    for artifact in deps {
        let jar_filename = format!(
            "{}-{}{}.jar",
            artifact.id, artifact.version, artifact.qualifier
        );
        let checksum_filename = jar_filename.clone() + ".sha1";
        let jar_path = jassets_dir.join(jar_filename.clone());

        let artifact_base_url = Url::from_str(MAVEN_CENTRAL_BASE_URL)
            .unwrap()
            .join(
                format!(
                    "{}/{}/{}/",
                    artifact.group.replace(".", "/"),
                    artifact.id,
                    artifact.version
                )
                .as_str(),
            )
            .unwrap();
        let jar_url = artifact_base_url.clone().join(&jar_filename).unwrap();
        let checksum_url = artifact_base_url.join(&checksum_filename).unwrap();

        if let Err(err) = download_file(jar_url, &jar_path) {
            println!(
                "cargo::error=Failed to download jar file: {}",
                &jar_filename
            );
            println!("cargo::error={err}");
            return;
        }

        let checksum = match fetch_jar_checksum(checksum_url) {
            Ok(sum) => sum,
            Err(err) => {
                println!("cargo::error=Failed to get checksum for {}", &jar_filename);
                println!("cargo::error={err}");
                return;
            }
        };

        if let Err(err) = verify_download(&jar_path, &checksum) {
            println!("cargo::error={err}");
            return;
        }
    }
}

fn read_dependency_list(deps_file: &Path) -> Result<Vec<MavenArtifact>, Box<dyn Error>> {
    let json = std::fs::read_to_string(deps_file)?;
    let deps_file: MavenDependenciesFile = serde_json::from_str(&json)?;

    Ok(deps_file
        .artifacts
        .into_iter()
        .map(|a| {
            let parts = a.splitn(3, ":").collect::<Vec<&str>>();

            MavenArtifact {
                group: parts[0].to_owned(),
                id: parts[1].to_owned(),
                version: parts[2].to_owned(),
                qualifier: "".to_owned(),
            }
        })
        .collect())
}

fn download_file(url: Url, dest: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.error_for_status()?;
    let bytes = response.bytes()?;

    let mut file = File::create(dest)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn verify_download(path: &Path, checksum: &str) -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;

    let sha1 = sha1_smol::Sha1::from(&buffer);
    let digest = sha1.hexdigest();

    if digest == checksum.trim() {
        Ok(())
    } else {
        Err(format!(
            "Downloaded artifact does not match the provided checksum for {:?}",
            path.file_name().unwrap()
        )
        .into())
    }
}

fn fetch_jar_checksum(url: Url) -> Result<String, Box<dyn Error>> {
    eprintln!("url: {:?}", url);
    Ok(reqwest::blocking::get(url)?.error_for_status()?.text()?)
}
