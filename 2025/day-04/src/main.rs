mod structs;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rayon::{prelude::*, result};

fn parse_input(input: &str) -> Vec<Vec<i32>> { 
    input.trim_end()
        .lines()
        .map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => 0,
                    '@' => 1,
                    _ => panic!("unexpected char in input"),
                }
            }).collect()
        })
        .collect()
 }

fn solve_puzzle1(matrix: &Vec<Vec<i32>>) -> i32 {
    let row_count = matrix.len();
    let col_count = matrix[0].len();

    let mut result_matrix = matrix.clone();

    result_matrix.iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..col_count {
            let mut adjacency = 0;
            let mut kernel_indices: Vec<usize> = Vec::new();
            let mut kernel: Vec<i32> = Vec::new();

            let start_k = if i == 0 { 0 } else { i - 1 };
            for k in start_k..(i + 2).min(row_count) {
                let start_l = if j == 0 { 0 } else { j - 1 };
                for l in start_l..(j + 2).min(col_count) {
                    kernel_indices.push(k*10+l);
                    kernel.push(matrix[k][l]);
                    adjacency += matrix[k][l];
                }
            }

            println!("kernel: {:?}, {:?}, {:?}, i{:?}, j{:?}", kernel_indices, kernel, adjacency, i, j);
            if adjacency < 5 && row[j] == 1 {
                row[j] = 1;
            } else {
                row[j] = 0;
            }
        }
    });

    println!("matrix");
    for row in matrix {
        println!("{:?}", row)
    }
    println!("result_matrix");
    for row in &result_matrix {
        println!("{:?}", row)
    }

    result_matrix.iter().flatten().sum()
}

fn solve_puzzle2(matrix: &Vec<Vec<i32>>) -> i32 { 0 }

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
        const EXAMPLE_OUTPUT: i32 = 13;
        let parsed_input = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle1(&parsed_input);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i32 = 6;
        let parsed_input = parse_input(EXAMPLE_INPUT);
        let result = solve_puzzle2(&parsed_input);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
