mod utils;

type Calorie = u32;

pub fn main() {
    println!("{}", most_calories("data/day1/calories.txt", 1));
    println!("{}", most_calories("data/day1/calories.txt", 3));
}

// Attempts to convert a string to a calorie.
fn convert_to_calorie(line: &str) -> Calorie {
    line.parse::<Calorie>().map_or_else(
        |err| panic!("Could not convert line to calorie: {err}"),
        |calorie| calorie,
    )
}

fn most_calories(filename: &str, amount: usize) -> Calorie {
    let mut elf_calories = utils::read_file(filename)
        .split("\n\n") // Splits into calorie groups.
        .map(|calorie_group| {
            calorie_group
                .lines() // Splits calorie groups into lines.
                .map(convert_to_calorie) // Attempt convert each line to calorie.
                .sum::<Calorie>() // Sums the group of calories.
        })
        .collect::<Vec<Calorie>>(); // Collects all summed calories into a Vector of elves.

    elf_calories.sort_unstable(); // Sorts the vector.
    elf_calories.into_iter().rev().take(amount).sum() // Returns sum of largest (n) elves.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(24_000, most_calories("data/day1/calories-ex.txt", 1));
        assert_eq!(67_622, most_calories("data/day1/calories.txt", 1));
    }

    #[test]
    fn part2() {
        assert_eq!(45_000, most_calories("data/day1/calories-ex.txt", 3));
        assert_eq!(201_491, most_calories("data/day1/calories.txt", 3));
    }
}
