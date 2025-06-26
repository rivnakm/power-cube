use serde_repr::Serialize_repr;
use serde_with::serde_as;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[cfg_attr(test, derive(PartialEq))]
#[repr(u8)]
pub enum FaceColor {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
}

impl From<FaceName> for FaceColor {
    fn from(value: FaceName) -> Self {
        match value {
            FaceName::Up => FaceColor::White,
            FaceName::Down => FaceColor::Yellow,
            FaceName::Left => FaceColor::Orange,
            FaceName::Right => FaceColor::Red,
            FaceName::Front => FaceColor::Green,
            FaceName::Back => FaceColor::Blue,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FaceName {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

#[serde_as]
#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Cube<const N: usize> {
    #[serde_as(as = "[[[_; N]; N]; 6]")]
    faces: [[[FaceColor; N]; N]; 6],
}

impl<const N: usize> Default for Cube<N> {
    fn default() -> Self {
        Cube {
            faces: [
                [[FaceName::Up.into(); N]; N],
                [[FaceName::Down.into(); N]; N],
                [[FaceName::Left.into(); N]; N],
                [[FaceName::Right.into(); N]; N],
                [[FaceName::Front.into(); N]; N],
                [[FaceName::Back.into(); N]; N],
            ],
        }
    }
}

impl<const N: usize> Cube<N> {
    pub fn from_tnoodle(tnoodle_cube: &str) -> Self {
        dbg!(&tnoodle_cube);
        const TNOODLE_FACE_ORDER: [FaceName; 6] = [
            FaceName::Up,
            FaceName::Right,
            FaceName::Front,
            FaceName::Down,
            FaceName::Left,
            FaceName::Back,
        ];

        let mut tnoodle_chars = tnoodle_cube.chars();
        let mut cube = Self::default();

        for face in TNOODLE_FACE_ORDER {
            for i in 0..N {
                for j in 0..N {
                    let sticker = match tnoodle_chars.next().unwrap() {
                        'U' => FaceColor::from(FaceName::Up),
                        'D' => FaceColor::from(FaceName::Down),
                        'L' => FaceColor::from(FaceName::Left),
                        'R' => FaceColor::from(FaceName::Right),
                        'F' => FaceColor::from(FaceName::Front),
                        'B' => FaceColor::from(FaceName::Back),
                        _ => panic!("Unrecognized sticker color"),
                    };

                    cube.faces[face as usize][j][i] = sticker;
                }
            }
        }

        cube
    }
}

pub type ThreeCube = Cube<3>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tnoodle_default() {
        let tnoodle = "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB";

        let cube = ThreeCube::from_tnoodle(tnoodle);

        assert_eq!(cube, ThreeCube::default());
    }
}
