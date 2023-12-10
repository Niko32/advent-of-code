mod structs;
use crate::structs::*;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

pub fn solve_puzzle1(input: &str) -> i64 {
    let races = Races::from(input);
    races
        .vec
        .into_iter()
        .map(|race| race.n_winning_strategies())
        .product::<usize>() as i64
}

pub fn solve_puzzle2(input: &str) -> i64 {
    let race = Race::from(input);
    race.n_winning_strategies() as i64
}

fn main() {
    const INPUT: &str = include_str!("../data/input1.txt");

    let solution1 = solve_puzzle1(INPUT);
    println!("Solution to puzzle one: {}", solution1);

    let solution2 = solve_puzzle2(INPUT);
    println!("Solution to puzzle two: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input1() {
        const EXAMPLE_OUTPUT: i64 = 288;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i64 = 71503;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
