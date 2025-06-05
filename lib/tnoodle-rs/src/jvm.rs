use std::path::Path;

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

pub fn get_jvm(base_path: Option<&Path>) -> Result<Jvm, J4RsError> {
    let jvm = match base_path {
        Some(base_path) => JvmBuilder::new()
            .with_base_path(base_path.to_str().unwrap())
            .build()?,
        None => JvmBuilder::new().build()?,
    };

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
        let jvm = get_jvm(None);

        assert!(jvm.is_ok());
    }
}
