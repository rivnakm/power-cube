use j4rs::{Instance, InvocationArg, Jvm, errors::J4RsError};

use crate::Scramble;

pub enum PuzzleType {
    Three,
}

pub struct Puzzle {
    jvm: Jvm,
    rand: Instance,
    registry: Instance,
}

impl Puzzle {
    pub fn new(jvm: Jvm, puzzle_type: PuzzleType) -> Result<Puzzle, J4RsError> {
        let registry_enum =
            jvm.static_class("org.worldcubeassociation.tnoodle.scrambles.PuzzleRegistry")?;

        let enum_field = match puzzle_type {
            PuzzleType::Three => "THREE",
        };

        let rand = jvm.invoke_static(
            "org.worldcubeassociation.tnoodle.scrambles.Puzzle",
            "getSecureRandom",
            InvocationArg::empty(),
        )?;

        let registry = jvm.field(&registry_enum, enum_field)?;

        Ok(Puzzle {
            jvm,
            rand,
            registry,
        })
    }

    pub fn generate_wca_scramble(&self) -> Result<Scramble, J4RsError> {
        let puzzle = self
            .jvm
            .invoke(&self.registry, "getScrambler", InvocationArg::empty())?;
        let rand = self.jvm.clone_instance(&self.rand)?;

        let scramble =
            self.jvm
                .invoke(&puzzle, "generateWcaScramble", &[InvocationArg::from(rand)])?;

        self.jvm.to_rust(scramble)
    }

    pub fn scramble_cube(&self, scramble: Scramble) -> Result<String, J4RsError> {
        let puzzle = self
            .jvm
            .invoke(&self.registry, "getScrambler", InvocationArg::empty())?;
        let solved_state = self
            .jvm
            .invoke(&puzzle, "getSolvedState", InvocationArg::empty())?;

        let scrambled_state =
            scramble
                .split_whitespace()
                .try_fold(solved_state, |state, mov| {
                    self.jvm
                        .invoke(&state, "apply", &[InvocationArg::try_from(mov)?])
                })?;

        let scrambled_state = self.jvm.cast(
            &scrambled_state,
            "org.worldcubeassociation.tnoodle.puzzle.CubePuzzle$CubeState",
        )?;

        let face_cube = self
            .jvm
            .invoke(&scrambled_state, "toFaceCube", InvocationArg::empty())?;

        self.jvm.to_rust(face_cube)
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
        let puzzle = Puzzle::new(jvm, PuzzleType::Three).unwrap();
        let scramble = puzzle
            .generate_wca_scramble()
            .expect("failed to generate scramble");

        assert!(!scramble.is_empty());
    }

    #[test]
    fn test_scramble_cube() {
        let jvm = get_jvm(None).unwrap();
        let puzzle = Puzzle::new(jvm, PuzzleType::Three).unwrap();
        let scrambled_cube = puzzle.scramble_cube("U2 D2 L2 R2 F2 B2".into()).unwrap();

        assert_eq!(
            scrambled_cube,
            "UDUDUDUDURLRLRLRLRFBFBFBFBFDUDUDUDUDLRLRLRLRLBFBFBFBFB"
        )
    }
}
