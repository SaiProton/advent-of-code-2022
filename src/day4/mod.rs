use std::{fmt::Display, str::FromStr};

// Holds the contents of each of the inputs files.
const PAIRS_TEST: &str = include_str!("pairs-ex.txt");
const PAIRS_REAL: &str = include_str!("pairs.txt");

type Pair<T> = (T, T);

pub fn main() {
    println!("{}", pair_comparison(PAIRS_TEST, &range_contains));
    println!("{}", pair_comparison(PAIRS_REAL, &range_contains));
}

fn pair_comparison(file_contents: &str, range_fn: &dyn Fn(&Pair<Pair<u32>>) -> bool) -> u32 {
    file_contents
        .lines() // Splits the list into individual lines of assignments.
        .map(to_pairs) // Splits line into a pair of two elves.
        .map(|elves| to_endpoint_pairs(&elves)) // Splits each elf into a pair of endpoints.
        .map(|(elf_1, elf_2)| {
            (
                // Converts each of the pairs of endpoints to integers.
                parse_endpoints::<u32>(&elf_1),
                parse_endpoints::<u32>(&elf_2),
            )
        })
        .map(|elves| range_fn(&elves)) // Applies custom fn to use for comparing pairs.
        .map(u32::from) // Converts booleans to integers.
        .sum() // Returns total amount of elf pairs that pass 'range_fn'
}

fn to_pairs(line: &str) -> Pair<String> {
    // Attempts to split the line by a comma.
    line.split_once(',').map_or_else(
        || panic!("Could not split line into pair, no ',' found."),
        |pair| (pair.0.to_owned(), pair.1.to_owned()),
    )
}

fn to_endpoint_pairs(elves: &Pair<String>) -> Pair<Pair<String>> {
    // Attempts to split each elf into a pair of endpoints by '-'
    (
        elves.0.split_once('-').map_or_else(
            || panic!("Could not split of pair, no '-' found."),
            |endpoints| (endpoints.0.to_owned(), endpoints.1.to_owned()),
        ),
        elves.1.split_once('-').map_or_else(
            || panic!("Could not split second pair, no '-' found."),
            |endpoints| (endpoints.0.to_owned(), endpoints.1.to_owned()),
        ),
    )
}

fn parse_endpoints<T>(endpoints: &Pair<String>) -> Pair<T>
where
    T: FromStr + Display,
    <T as FromStr>::Err: Display,
{
    // Attempts to parse a pair of endpoints to a defnied type.
    (
        endpoints.0.parse().map_or_else(
            |err| panic!("Could not convert first endpoint: {err}"),
            |endpoint| endpoint,
        ),
        endpoints.1.parse().map_or_else(
            |err| panic!("Could not convert second endpoint: {err}"),
            |endpoint| endpoint,
        ),
    )
}

fn range_contains<T: PartialOrd>((elf_1, elf_2): &Pair<Pair<T>>) -> bool {
    // Checks for if elf_1's range encompasses elf_2, or if elf_2's range encompasses elf_1's.
    (elf_1.0 <= elf_2.0) && (elf_2.1 <= elf_1.1) || (elf_2.0 <= elf_1.0 && elf_1.1 <= elf_2.1)
}

// fn range_overlaps<T: PartialOrd>((elf_1, elf_2): &Pair<Pair<T>>) -> bool {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(2, pair_comparison(PAIRS_TEST, &range_contains));
        assert_eq!(560, pair_comparison(PAIRS_REAL, &range_contains));
    }

    // #[test]
    // fn part2() {
    //     todo!()
    // }
}
