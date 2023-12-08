mod structs;
use crate::structs::*;

const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

pub fn solve_puzzle1(input: &str) -> i32 {
    let almanac = Almanac::from(EXAMPLE_INPUT);
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
