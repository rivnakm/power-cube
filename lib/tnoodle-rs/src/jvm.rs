use j4rs::{Jvm, JvmBuilder, MavenArtifact, errors::J4RsError};
use lazy_static::lazy_static;
use serde::Deserialize;

const MAVEN_DEPS_JSON: &str = include_str!("../maven-deps.json");
lazy_static! {
    static ref MAVEN_DEPS: MavenDependencies = serde_json::from_str(MAVEN_DEPS_JSON).unwrap();
}

#[derive(Deserialize)]
struct MavenDependencies {
    artifacts: Vec<String>,
}

pub fn get_jvm() -> Result<Jvm, J4RsError> {
    // TODO: need to add a param to set the jvm builder base path so I can set it to the appropriate
    // tauri resource directory which is known at runtime
    // https://tauri.app/develop/resources/#accessing-files-in-rust
    // https://github.com/astonbitecode/j4rs?tab=readme-ov-file#portability-assumptions-after-rust-build-shipping-a-j4rs-application
    let jvm = JvmBuilder::new().build()?;

    for artifact_name in MAVEN_DEPS.artifacts.iter() {
        jvm.deploy_artifact(&MavenArtifact::from(artifact_name.as_str()))?;
    }

    Ok(jvm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_jvm() {
        let jvm = get_jvm();

        assert!(jvm.is_ok());
    }
}
