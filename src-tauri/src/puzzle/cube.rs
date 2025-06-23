use serde_with::serde_as;

use serde::Serialize;

use super::{MoveSequence, Puzzle, PuzzleMove};

#[derive(Clone, Copy, Debug, Serialize)]
pub enum FaceColor {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
}

pub enum FaceName {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

#[serde_as]
#[derive(Serialize)]
pub struct Cube<const N: usize> {
    #[serde_as(as = "[[[_; N]; N]; 6]")]
    faces: [[[FaceColor; N]; N]; 6],
}

impl<const N: usize> Default for Cube<N> {
    fn default() -> Self {
        Cube {
            faces: [
                [[FaceColor::White; N]; N],
                [[FaceColor::Yellow; N]; N],
                [[FaceColor::Orange; N]; N],
                [[FaceColor::Red; N]; N],
                [[FaceColor::Green; N]; N],
                [[FaceColor::Blue; N]; N],
            ],
        }
    }
}

pub type ThreeCube = Cube<3>;

impl Puzzle for ThreeCube {
    type PuzzleMove = ThreeCubeMove;

    fn exec(&mut self, puzzle_move: &Self::PuzzleMove) {
        todo!()
    }

    fn exec_seq(&mut self, puzzle_move_seq: &MoveSequence<Self, Self::PuzzleMove>) {
        for m in puzzle_move_seq {
            self.exec(m);
        }
    }
}

impl From<&str> for MoveSequence<ThreeCube, ThreeCubeMove> {
    fn from(value: &str) -> Self {
        let moves = value
            .split(" ")
            .flat_map(|m| {
                if m.ends_with('2') {
                    let single = m.trim_end_matches("2");

                    vec![single, single]
                } else {
                    vec![m]
                }
            })
            .map(|m| m.into())
            .collect();

        MoveSequence {
            moves,
            puzzle: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ThreeCubeMove {
    Up,
    UpReverse,
    Down,
    DownReverse,
    Left,
    LeftReverse,
    Right,
    RightReverse,
    Front,
    FrontReverse,
    Back,
    BackReverse,
}

// TODO: also implement TryFrom
impl From<&str> for ThreeCubeMove {
    fn from(value: &str) -> Self {
        match value {
            "U" => ThreeCubeMove::Up,
            "U'" => ThreeCubeMove::UpReverse,
            "D" => ThreeCubeMove::Down,
            "D'" => ThreeCubeMove::DownReverse,
            "L" => ThreeCubeMove::Left,
            "L'" => ThreeCubeMove::LeftReverse,
            "R" => ThreeCubeMove::Right,
            "R'" => ThreeCubeMove::RightReverse,
            "F" => ThreeCubeMove::Front,
            "F'" => ThreeCubeMove::FrontReverse,
            "B" => ThreeCubeMove::Back,
            "B'" => ThreeCubeMove::BackReverse,
            _ => panic!("Invalid 3x3 move: {}", value),
        }
    }
}

impl PuzzleMove<ThreeCube> for ThreeCubeMove {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("U", ThreeCubeMove::Up)]
    #[test_case("U'", ThreeCubeMove::UpReverse)]
    #[test_case("D", ThreeCubeMove::Down)]
    #[test_case("D'", ThreeCubeMove::DownReverse)]
    #[test_case("L", ThreeCubeMove::Left)]
    #[test_case("L'", ThreeCubeMove::LeftReverse)]
    #[test_case("R", ThreeCubeMove::Right)]
    #[test_case("R'", ThreeCubeMove::RightReverse)]
    #[test_case("F", ThreeCubeMove::Front)]
    #[test_case("F'", ThreeCubeMove::FrontReverse)]
    #[test_case("B", ThreeCubeMove::Back)]
    #[test_case("B'", ThreeCubeMove::BackReverse)]
    fn three_cube_move_from_str(move_str: &'static str, expected: ThreeCubeMove) {
        assert_eq!(expected, ThreeCubeMove::from(move_str));
    }

    #[test]
    fn three_cube_move_sec_from_string_normalizes() {
        let seq_str = "F2 D' B2";
        let expected = vec![
            ThreeCubeMove::Front,
            ThreeCubeMove::Front,
            ThreeCubeMove::DownReverse,
            ThreeCubeMove::Back,
            ThreeCubeMove::Back,
        ];

        let seq = MoveSequence::<ThreeCube, ThreeCubeMove>::from(seq_str);
        assert_eq!(seq.moves, expected);
    }
}
