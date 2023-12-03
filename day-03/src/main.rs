mod structs;
use crate::structs::*;

pub fn solve_puzzle1(input: &str) -> i32 {
    let padded_input: PaddedSchematic = PaddedSchematic::from(input);
    padded_input
        .part_numbers()
        .into_iter()
        .sum()
}

pub fn solve_puzzle2(input: &str) -> i32 {
    let padded_input: PaddedSchematic = PaddedSchematic::from(input);
    padded_input 
        .find_gears()
        .into_iter()
        .map(|gear| { gear.ratio() })
        .sum()
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
        const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        const EXAMPLE_OUTPUT: i32 = 4361;
        let result = solve_puzzle1(EXAMPLE_INPUT); 
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        const EXAMPLE_OUTPUT: i32 = 467835;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}