use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Calories = i32;
type Elf = Vec<Calories>;

// Utility function for getting the lines of a file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Preprocesses from the file contents to a Vector of elves.
fn preprocess_calories<P>(filename: P) -> io::Result<Vec<Elf>>
where
    P: AsRef<Path>,
{
    let mut elves: Vec<Elf> = vec![];
    let mut temp: Elf = vec![];

    for line in read_lines(filename)? {
        // If the line was successfully parsed, add it to the current inner vector,
        // Otherwise, add it to the main el
        if let Ok(parsed_line) = line?.parse::<Calories>() {
            temp.push(parsed_line);
        } else {
            elves.push(temp);
            temp = vec![];
        }
    }

    if !temp.is_empty() {
        elves.push(temp);
    }

    Ok(elves)
}

fn most_calories(filename: &str) -> Vec<i32> {
    // let elves = preprocess_calories(filename).expect("Could not read calories...");
    let elves = match preprocess_calories(filename) {
        Ok(e) => e,
        Err(e) => panic!("{}", e),
    };

    println!("{elves:?}");

    elves[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            vec![1000, 2000, 3000],
            most_calories("data/day1/calories-ex.txt")
        );
    }

    #[test]
    fn part2() {
        assert_eq!(1, 1);
    }
}
