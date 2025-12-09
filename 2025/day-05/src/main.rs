mod structs;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
// use rayon::prelude::*;

fn parse_input(input: &str) -> (Vec<[u64; 2]>, Vec<u64>) { 
    let parts: Vec<&str> = if input.contains("\r\n\r\n") {
        input.trim_end().split("\r\n\r\n").collect()
    } else {
        input.trim_end().split("\n\n").collect()
    };
    let (range_lines, id_lines) = (parts[0], parts[1]);

    let ranges: Vec<[u64; 2]> = range_lines
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line
                .split("-")
                .map(|line| {
                    line.parse::<u64>().expect("failed to parse ranges as u64")
                })
                .collect();
            [nums[0], nums[1]]
        })
        .collect();

    let ids: Vec<u64> = id_lines
        .lines()
        .map(|line| {
            line.parse().expect("failed to parse id as u64")
        })
        .collect();

    return (ranges, ids)
}

fn solve_puzzle1(ranges: &Vec<[u64; 2]>, ids: &Vec<u64>) -> u64 {
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

fn solve_puzzle2(ranges: &Vec<[u64; 2]>, _: &Vec<u64>) -> u64 {
    let mut ranges2 = ranges.clone();
    let mut i: usize = 0;
    let mut full_iteration: bool = true;

    loop { // Merge ranges together
        // println!("i: {}, ranges2.len(): {}", i, ranges2.len());
        if i+1 >= ranges2.len() {
            break
        }
        for j in i+1..ranges2.len() {
            // println!("  j: {}, ranges2: {:?}", j, ranges2);
            full_iteration = true;
            if ranges2[i][0] <= ranges2[j][1] && ranges2[i][1] >= ranges2[j][0] {
                ranges2[i][0] = ranges2[i][0].min(ranges2[j][0]);
                ranges2[i][1] = ranges2[i][1].max(ranges2[j][1]);
                ranges2.remove(j);
                full_iteration = false;
                break
            }
        }
        if full_iteration {
            i += 1;
        }
    }

    let mut fresh_count = 0;
    for range in ranges2 {
        fresh_count += 1 + range[1] - range[0];
    }

    fresh_count
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
        const EXAMPLE_OUTPUT: u64 = 3;
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle1(&ranges, &ids);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: u64 = 14;
        let (ranges, ids) = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle2(&ranges, &ids);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
