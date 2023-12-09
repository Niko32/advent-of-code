mod structs;
use crate::structs::*;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

pub fn solve_puzzle1(input: &str) -> i32 {
    let almanac = Almanac::from(input);
    almanac.nearest_seed_location() as i32
}

pub fn solve_puzzle2(input: &str) -> i32 {
    todo!();
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
        const EXAMPLE_OUTPUT: i32 = 35;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[ignore]
    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i32 = 30;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
