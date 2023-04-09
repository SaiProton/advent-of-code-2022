use crate::day5::Stack;

use super::traits::Poppable;

pub enum CraneMode {
    Mode9000,
    Mode9001,
}

impl CraneMode {
    pub fn perform_procedure(
        &self,
        stacks: &mut [Stack],
        moves: usize,
        source: usize,
        destination: usize,
    ) {
        match *self {
            Self::Mode9000 => Self::procedure_9000(stacks, source, destination),
            Self::Mode9001 => Self::procedure_9001(stacks, moves, source, destination),
        }
    }

    fn procedure_9000(stacks: &mut [Stack], source: usize, destination: usize) {
        let popped = stacks.get_mut(source - 1).map_or_else(
            || panic!("Could not get source stack during move"),
            |stack| stack.try_pop("Could not pop from stack during procedure 9000"),
        );

        stacks
            .get_mut(destination - 1)
            .map_or_else(
                || panic!("Could not get destination stack during move"),
                |dest| dest,
            )
            .push(popped);
    }

    fn procedure_9001(stacks: &mut [Stack], moves: usize, source: usize, destination: usize) {}
}
