mod structs;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::cell::UnsafeCell;
use rayon::prelude::*;

fn parse_input(input: &str) -> (Vec<[u32; 2]>, Vec<u32>) { 
    let parts: Vec<&str> = if input.contains("\r\n\r\n") {
        input.trim_end().split("\r\n\r\n").collect()
    } else {
        input.trim_end().split("\n\n").collect()
    };
    let (range_lines, id_lines) = (parts[0], parts[1]);

    let ranges: Vec<[u32; 2]> = range_lines
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line
                .split("-")
                .map(|line| {
                    line.parse::<u32>().expect("failed to parse ranges as u32")
                })
                .collect();
            [nums[0], nums[1]]
        })
        .collect();

    let ids: Vec<u32> = id_lines
        .lines()
        .map(|line| {
            line.parse().expect("failed to parse id as u32")
        })
        .collect();

    return (ranges, ids)
}

fn solve_puzzle1(ranges: &Vec<[u32; 2]>, ids: &Vec<u32>) -> u32 {
    let mut fresh_counter = 0;
    for id in ids {
        for range in ranges{
            if *id >= range[0] && *id <= range[1] {
                fresh_counter += 1;
                break
            }
        }
    }
    fresh_counter
}

fn solve_puzzle2(ranges: &Vec<[u32; 2]>, _: &Vec<u32>) -> u32 {
    let mut fresh_ids = UnsafeCell::new([false; u32::MAX as usize]);
    ranges.par_iter().for_each(|range| {
        unsafe {
            let ptr = fresh_ids.get();
            for i in range[0]..=range[1] {
                (*ptr)[i as usize] = true;
            }
        }
    });
    fresh_ids.into_inner().iter().filter(|x|**x).count() as u32
}

fn main() {
    const INPUT: &str = include_str!("../data/input1.txt");

    let mut file = File::create("../times.txt")
        .expect("failed to create file handle");

    let parse_timer = Instant::now();
    let (ranges, ids) = parse_input(INPUT);
    let parse_time = parse_timer.elapsed();

    let timer1 = Instant::now();
    let solution1 = solve_puzzle1(&ranges, &ids);
    let time1 = timer1.elapsed();
    println!("Solution to puzzle one: {}", solution1);

    let timer2 = Instant::now();
    let solution2 = solve_puzzle2(&ranges, &ids);
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
        const EXAMPLE_OUTPUT: u32 = 3;
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle1(&ranges, &ids);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: u32 = 14;
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle2(&ranges, &ids);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
