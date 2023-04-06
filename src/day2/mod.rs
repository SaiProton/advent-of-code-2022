use std::path::Path;

use crate::utils;

const ASCII_NORMAL: u8 = b"A"[0];
const PLAYER_ASCII_DIFF: u8 = b"X"[0] - b"A"[0];

type Score = u32;

fn rock_paper_scissors<P>(filename: P) -> Score
where
    P: AsRef<Path>,
{
    // Takes each line of the strategy, computes the score, and takes the sum.
    utils::read_file(filename).lines().map(play_round).sum()
}

// Takes a line and plays a single round of RPS with it.
fn play_round(play: &str) -> Score {
    // Splits the line into the opponent and player moves.
    match play.split_whitespace().collect::<Vec<_>>()[..] {
        [opponent, player] => preprocess_plays(opponent, player),
        _ => panic!("The line must have exactly two characters seperated by whitespace"),
    }
    .into_iter()
    .reduce(determine_winner) // Determines the winner given the two plays.
    .map_or_else(|| panic!("Empty line encountered"), |score| score)
    .into()
}

// Takes the individual characters of a play as strings,
// then preprocesses them to be normalized ascii values.
fn preprocess_plays(opponent: &str, player: &str) -> [u8; 2] {
    [
        // 'A' maps to 0, 'B' maps to 1, 'C' maps to 2
        (opponent.as_bytes().first().map_or_else(
            || panic!("Could not convert the opponent character to an ascii value"),
            |ascii| ascii,
        ) - ASCII_NORMAL),
        // 'X' maps to 0, 'Y' maps to 1, 'Z' maps to 2
        (player.as_bytes().first().map_or_else(
            || panic!("Could not convert the opponent character to an ascii value"),
            |ascii| ascii,
        ) - (ASCII_NORMAL + PLAYER_ASCII_DIFF)),
    ]
}

// Takes the opponent and player's character ascii values, determines the winner,
// and returns the appropriate score.
const fn determine_winner(opponent: u8, player: u8) -> u8 {
    // The ascii values must be converted to integers, since the solution involves working with
    // negative numbers, and we don't want to overflow.
    if (opponent as i32 - 1).rem_euclid(3) == player as i32 {
        player + 1
    } else if (opponent as i32 + 1).rem_euclid(3) == player as i32 {
        6 + player + 1
    } else {
        3 + player + 1
    }
}

pub fn main() {
    println!("{}", rock_paper_scissors("data/day2/strategy-ex.txt"));
    println!("{}", rock_paper_scissors("data/day2/strategy.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(15, rock_paper_scissors("data/day2/strategy-ex.txt"));
        assert_eq!(11_386, rock_paper_scissors("data/day2/strategy.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(1, 1);
    }
}
