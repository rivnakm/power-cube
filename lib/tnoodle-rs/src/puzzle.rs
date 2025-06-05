use j4rs::{Instance, InvocationArg, Jvm, errors::J4RsError};

pub enum PuzzleType {
    Three,
}

pub struct Puzzle {
    jvm: Jvm,
    registry: Instance,
}

impl Puzzle {
    pub fn new(jvm: Jvm, puzzle_type: PuzzleType) -> Result<Puzzle, J4RsError> {
        let registry_enum =
            jvm.static_class("org.worldcubeassociation.tnoodle.scrambles.PuzzleRegistry")?;
        let enum_field = match puzzle_type {
            PuzzleType::Three => "THREE",
        };
        let registry = jvm.field(&registry_enum, enum_field)?;

        Ok(Puzzle { jvm, registry })
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

#[cfg(test)]
mod tests {
    use crate::jvm::get_jvm;

    use super::*;

    #[test]
    fn test_puzzle_new() {
        let jvm = get_jvm(None).unwrap();
        let registry = Puzzle::new(jvm, PuzzleType::Three);

        assert!(registry.is_ok());
    }

    #[test]
    fn test_generate_wca_scramble() {
        let jvm = get_jvm(None).unwrap();
        let registry = Puzzle::new(jvm, PuzzleType::Three).unwrap();
        let scramble = registry
            .generate_wca_scramble()
            .expect("failed to generate scramble");

        assert!(!scramble.is_empty());

        println!("scramble: {}", scramble);
    }
}
