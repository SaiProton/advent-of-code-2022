use std::vec;

// Holds the contents of each of the inputs files.
const INSTRUCTIONS_TEST: &str = include_str!("instructions-ex.txt");
const INSTRUCTIONS_REAL: &str = include_str!("instructions.txt");

const ITEM_CHAR_SPACING: usize = 4;

type Instruction = u32;
type Stack = Vec<char>;
type Procedure = Vec<Instruction>;

pub fn main() {
    let mut operator = CraneOperator::new();

    operator.load_instructions(INSTRUCTIONS_REAL);
    operator.load_instructions(INSTRUCTIONS_TEST);

    let mut vec = vec![1, 2, 3, 4, 5];
    for el in &mut vec {
        *el *= 2;
    }

    println!("{}", operator.rearrange_stacks());
}

/// `CraneOperator` struct.
/// * `stacks` - Vector of stacks that will be manipulated according to the procuedures.
/// * `procedures` - Vector of procedures composing of Instructions to be carried out.
struct CraneOperator {
    stacks: Vec<Stack>,
    procedures: Vec<Procedure>,
}

impl CraneOperator {
    /// Creates a new `CraneOperator` with empty vectors as default for `stacks` and `procedures`.
    pub const fn new() -> Self {
        Self {
            stacks: vec![],
            procedures: vec![],
        }
    }

    /// Loads instructions from `file_contents` into `stacks` and `procedures accordingly`.
    fn load_instructions(&mut self, file_contents: &str) {
        // Seperates the two sections by finding an empty line as a divider.
        let (items_section, procedures_section) = file_contents.split_once("\n\n").map_or_else(
            || panic!("Could not split the contents into two segments: No empty line found."),
            |sections| sections,
        );

        // Set to be populated with an appropriate amount of new stacks to hold the items.
        self.stacks = vec![Stack::new(); Self::get_stack_count(items_section)];
        // Set procedures to be an empty vector.
        self.procedures = vec![];

        // Loads the items into stacks.
        Self::load_items(items_section, &mut self.stacks);
        // Loads the procedures into procedures.
        Self::load_procedures(procedures_section, &mut self.procedures);
    }

    /// Gets the appropriate amount of stacks to hold the items in `items_section`.
    fn get_stack_count(items_section: &str) -> usize {
        (items_section
            .lines() 
            .next() // First line of the items section
            .map_or_else(
                || panic!("Could not read contents of the 'items' segment"),
                |line| line,
            )
            .len() // Length of first line.
            + 1)
            / ITEM_CHAR_SPACING // Divide by horizontal spacing between each item character.
    }

    /// Loads items from `items_section` into `stacks`.
    fn load_items(items_section: &str, stacks: &mut [Stack]) {
        // Pushes an item to the stacks given an index `i` and item `item`.
        let mut push_item = |i: usize, item: char| {
            stacks
                .get_mut(i) // Attempts to get corresponding mutable stack.
                .map_or_else(
                    || panic!("Attempted to index out of bounds item when loading items"),
                    |stack| stack,
                )
                .push(item);
        };

        for line in items_section.lines() {
            // Iterating over the items by stepping by the item spacing and skipping the first char. 
            for (i, item) in line.chars().skip(1).step_by(ITEM_CHAR_SPACING).enumerate() {
                // Only pushes to a stack if the current item is an uppercase letter.
                if item.is_uppercase() {
                    push_item(i, item);
                }
            }
        };

        // After adding all the items, reverse each of the stacks
        // so the items added first are 'on top' of the stack.
        for stack in stacks {
            stack.reverse();
        }
    }

    /// Loads procedures from `procedures_section` into `procedures`.
    fn load_procedures(procedures_section: &str, procedures: &mut Vec<Procedure>) {
        // Parses an `instruction` in string form to an `Instruction` type.
        let parse_instruction = |instruction: &str| {
            instruction.parse::<Instruction>().map_or_else(
                |err| panic!("Could not convert string {instruction} to instruction: {err}"),
                |converted| converted,
            )
        };

        for line in procedures_section.lines() {
            procedures.push(
                line.split(' ') // Splitting by whitespace horizontally.
                    .skip(1) // Skipping the first word.
                    .step_by(2) // Every other element (to ignore the words).
                    .map(parse_instruction) // Parses the element to an `Instruction`.
                    .collect(), // Collects into a `Procedure`.
            );
        };
    }

    fn rearrange_stacks(&self) -> String {
        format!("{:?}\n{:?}", self.stacks, self.procedures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut operator = CraneOperator::new();
        operator.load_instructions(INSTRUCTIONS_TEST);

        assert_eq!("CMZ", operator.rearrange_stacks());
        // assert_eq!(560, pair_comparison(PAIRS_REAL, &range_contains));
    }

    // #[test]
    // fn part2() {
    //     assert_eq!(4, pair_comparison(PAIRS_TEST, &range_overlaps));
    //     assert_eq!(839, pair_comparison(PAIRS_REAL, &range_overlaps));
    // }
}
