mod structs;
use crate::structs::*;
mod errors;

pub fn solve_puzzle1(input: &str) -> i32 {
    let doc = ImprovedCalibrationDocument::from(input);
    doc.sum_calibration_values()
}

pub fn solve_puzzle2(input: &str) -> i32 {
    let mut doc = ImprovedCalibrationDocument::from(input);
    doc.parse_written_digits().sum_calibration_values()
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

        const EXAMPLE_INPUT: &str = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        const EXAMPLE_OUTPUT: i32 = 142;

        assert_eq!(solve_puzzle1(EXAMPLE_INPUT), EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_INPUT: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        const EXAMPLE_OUTPUT: i32 = 281;

        assert_eq!(solve_puzzle2(EXAMPLE_INPUT), EXAMPLE_OUTPUT)
    }
}