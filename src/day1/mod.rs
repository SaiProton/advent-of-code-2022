mod utils;

type Calorie = u32;

pub fn main() {
    println!("{}", most_calories("data/day1/calories.txt"));
}

fn convert_to_calorie(line: &str) -> Calorie {
    line.parse::<Calorie>().map_or_else(
        |err| panic!("Could not convert line to calorie: {err}"),
        |calorie| calorie,
    )
}

fn most_calories(filename: &str) -> Calorie {
    let most_calories = utils::read_file(filename)
        .split("\n\n") // Splits into calorie groups.
        .map(|calorie_group| {
            calorie_group
                .lines() // Splits calorie groups into lines.
                .map(convert_to_calorie) // Attempt convert each line to calorie.
                .sum::<Calorie>() // Sums the group of calories.
        })
        .max() // Finds the max of the summed calorie groups.
        .map_or_else(
            || panic!("Could not find maximum calorie count."),
            |max| max,
        );

    most_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(24_000, most_calories("data/day1/calories-ex.txt"));
        assert_eq!(67_622, most_calories("data/day1/calories.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(1, 1);
    }
}
