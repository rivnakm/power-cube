use j4rs::{Instance, InvocationArg, Jvm, errors::J4RsError};

pub enum PuzzleType {
    Three,
}

pub struct PuzzleRegistry {
    jvm: Jvm,
    registry: Instance,
}

impl PuzzleRegistry {
    pub fn new(puzzle_type: PuzzleType) -> Result<PuzzleRegistry, J4RsError> {
        let jvm = crate::jvm::get_jvm()?;
        let registry_enum =
            jvm.static_class("org.worldcubeassociation.tnoodle.scrambles.PuzzleRegistry")?;
        let enum_field = match puzzle_type {
            PuzzleType::Three => "THREE",
        };
        let registry = jvm.field(&registry_enum, enum_field)?;

        Ok(PuzzleRegistry { jvm, registry })
    }

    pub fn generate_wca_scramble(&self) -> Result<String, J4RsError> {
        let rand = self.jvm.invoke_static(
            "org.worldcubeassociation.tnoodle.scrambles.Puzzle",
            "getSecureRandom",
            InvocationArg::empty(),
        )?;

        let scrambler = self
            .jvm
            .invoke(&self.registry, "getScrambler", InvocationArg::empty())?;

        let scramble = self.jvm.invoke(
            &scrambler,
            "generateWcaScramble",
            &[InvocationArg::from(rand)],
        )?;

        self.jvm.to_rust(scramble)
    }
}

pub struct Puzzle {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_registry_new() {
        let registry = PuzzleRegistry::new(PuzzleType::Three);

        assert!(registry.is_ok());
    }

    #[test]
    fn test_generate_wca_scramble() {
        let registry = PuzzleRegistry::new(PuzzleType::Three).unwrap();
        let scramble = registry
            .generate_wca_scramble()
            .expect("failed to generate scramble");

        assert!(!scramble.is_empty());

        println!("scramble: {}", scramble);
    }
}
