use std::{path::PathBuf, thread};

use crossbeam::channel::{Receiver, RecvError};
use tnoodle_rs::{
    jvm,
    puzzle::{Puzzle, PuzzleType},
    Scramble,
};

pub struct BufferedScrambler {
    receiver: Receiver<Scramble>,
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
                sender.send(scramble).unwrap();
            }
        });

        BufferedScrambler { receiver }
    }

    pub fn generate_wca_scramble(&self) -> Result<Scramble, RecvError> {
        self.receiver.recv()
    }
}
