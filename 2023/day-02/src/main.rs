mod structs;
use crate::structs::*;

pub fn solve_puzzle1(input: &str) -> u32 {
    let total_cubes: Sample = Sample::new(12, 13, 14);

    let games = Games::from(input);
    let possible_games = games.possible_games_with(total_cubes);
    possible_games.into_iter()
        .map(|game| { game.id })
        .sum()
}

pub fn solve_puzzle2(input: &str) -> u32 {
    let games = Games::from(input);
    let minimum_sets_of_cubes: Vec<Sample> = games.0
        .into_iter()
        .map(|game| { Sample::union(&game.samples) })
        .collect();
    minimum_sets_of_cubes
        .into_iter()
        .map(|sample| { sample.power() })
        .sum::<u32>()
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
        const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        const EXAMPLE_OUTPUT: u32 = 8;
        let result = solve_puzzle1(EXAMPLE_INPUT); 
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        const EXAMPLE_OUTPUT: u32 = 2286;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}