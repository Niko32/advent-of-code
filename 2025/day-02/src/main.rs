mod structs;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

fn solve_puzzle1(input: &str) -> i32 {
    
}

fn solve_puzzle2(input: &str) -> i32 {
    
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
        const EXAMPLE_OUTPUT: i32 = 3;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i32 = 6;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
