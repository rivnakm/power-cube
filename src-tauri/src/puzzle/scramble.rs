use std::{path::PathBuf, thread};

use crossbeam::channel::{Receiver, RecvError};
use tnoodle_rs::{
    Scramble, jvm,
    puzzle::{Puzzle, PuzzleType},
};

use super::cube::ThreeCube;

pub struct BufferedScrambler {
    receiver: Receiver<(Scramble, ThreeCube)>,
}

const SCRAMBLE_BUFFER_SIZE: usize = 5;

impl BufferedScrambler {
    pub fn new(j4rs_dir: PathBuf) -> BufferedScrambler {
        let (sender, receiver) = crossbeam::channel::bounded(SCRAMBLE_BUFFER_SIZE);

        thread::spawn(move || {
            let jvm = jvm::get_jvm(Some(&j4rs_dir)).unwrap();
            let puzzle = Puzzle::new(jvm, PuzzleType::Three).unwrap();

            loop {
                let scramble = puzzle.generate_wca_scramble().unwrap();
                let scrambled_cube = puzzle.scramble_cube(scramble.clone()).unwrap();
                sender
                    .send((scramble, ThreeCube::from_tnoodle(&scrambled_cube)))
                    .unwrap();
            }
        });

        BufferedScrambler { receiver }
    }

    pub fn generate_wca_scramble(&self) -> Result<(Scramble, ThreeCube), RecvError> {
        self.receiver.recv()
    }
}
