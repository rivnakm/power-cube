use std::slice::Iter;

pub mod cube;
pub mod scramble;

pub trait Puzzle
where
    Self::PuzzleMove: PuzzleMove<Self>,
{
    type PuzzleMove;
    fn exec(&mut self, puzzle_move: &Self::PuzzleMove);
    fn exec_seq(&mut self, puzzle_move_seq: &MoveSequence<Self, Self::PuzzleMove>)
    where
        Self: std::marker::Sized;
}

pub trait PuzzleMove<P>
where
    P: Puzzle + ?Sized,
{
}

pub struct MoveSequence<P, M>
where
    P: Puzzle,
    M: PuzzleMove<P>,
{
    moves: Vec<M>,
    puzzle: std::marker::PhantomData<P>,
}

impl<'a, P, M> IntoIterator for &'a MoveSequence<P, M>
where
    P: Puzzle,
    M: PuzzleMove<P>,
{
    type Item = &'a M;

    type IntoIter = Iter<'a, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.as_slice().iter()
    }
}
