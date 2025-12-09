mod structs;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rayon::prelude::*;

fn parse_input(input: &str) -> Vec<[u64;2]> { 
    input
        .trim()
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line
                .split(",")
                .map(|s| {
                    s.parse().expect("failed to parse u64 from &str")
                })
                .collect::<Vec<u64>>();
            [nums[0], nums[1]]
        })
        .collect()
}

fn solve_puzzle1(coords: &Vec<[u64;2]>) -> u64 {
    coords
        .par_iter()
        .enumerate()
        .map(|(i, coord)| {
            let mut area: u64 = 0;
            for j in i..coords.len() {
                let width: u64 = if coord[0] > coords[j][0] { 1 + coord[0] - coords[j][0] } else { 1 + coords[j][0] - coord[0] };
                let height: u64 = if coord[1] > coords[j][1] { 1 + coord[1] - coords[j][1] } else { 1 + coords[j][1] - coord[1] };
                area = area.max(width * height);
            }
            area
        })
        .reduce(|| 0, |a, b| a.max(b))
}

fn solve_puzzle2(coords: &Vec<[u64;2]>) -> u64 {
    coords
        .iter()
        .enumerate()
        .map(|(i, coord)| {
            println!("i: {}", i);
            let mut area: u64 = u64::MAX;
            for j in 0..coords.len() {
                if coord[0] > coords[j][0] || coord[1] > coords[j][1] { continue }
                    let width: u64 = 1 + coords[j][0] - coord[0];
                    let height: u64 = 1 + coords[j][1] - coord[1];
                if width > 1 && height > 1 {
                    area = area.min(width * height);
                }
                println!("  j: {}, area: {}", j, area);
            }
            if area == u64::MAX {
                area = 0;
                for j in 0..coords.len() {
                if coord[0] > coords[j][0] || coord[1] > coords[j][1] { continue }
                    let width: u64 = 1 + coords[j][0] - coord[0];
                    let height: u64 = 1 + coords[j][1] - coord[1];
                    area = area.max(width * height);
                    println!("  j: {}, area: {}", j, area);
                }
                return area
            }
            area
        })
        .reduce( |a, b| a.max(b))
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../data/input1.txt");

    let mut file = File::create("../times.txt")
        .expect("failed to create file handle");

    let parse_timer = Instant::now();
    let parsed_input = parse_input(INPUT);
    let parse_time = parse_timer.elapsed();

    let timer1 = Instant::now();
    let solution1 = solve_puzzle1(&parsed_input);
    let time1 = timer1.elapsed();
    println!("Solution to puzzle one: {}", solution1);

    let timer2 = Instant::now();
    let solution2 = solve_puzzle2(&parsed_input);
    let time2 = timer2.elapsed();
    println!("Solution to puzzle two: {}", solution2);

    writeln!(file, "time for parsing:    {:?}\ntime for puzzle one: {:?}\ntime for puzzle two: {:?}", parse_time, time1, time2)
        .expect("failed to write to times.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

    #[test]
    fn test_example_input1() {
        const EXAMPLE_OUTPUT: u64 = 50;
        let parsed_input = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle1(&parsed_input);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: u64 = 24;
        let parsed_input = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle2(&parsed_input);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
