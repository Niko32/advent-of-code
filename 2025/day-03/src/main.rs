mod structs;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

fn solve_puzzle1(input: &str) -> i64 {
    let banks: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for bank in banks {
        let mut highest_i = 0;
        let mut highest_digit = 0;
        let mut second_highest_digit = 0;

        for (i, c) in bank.chars().take(bank.chars().count() - 1).enumerate() {
            let digit = c.to_digit(10).expect("failed to cast to digit");
            if digit > highest_digit {
                highest_digit = digit;
                highest_i = i;
            }
        }

        for (i, c) in bank.chars().enumerate() {
            let digit = c.to_digit(10).expect("failed to cast to digit");
            if i > highest_i && digit > second_highest_digit {
                second_highest_digit = digit;
            }
        }

        let mut number = highest_digit.to_string();
        println!("number: {:?}", number);
        println!("second_highest_digit: {:?}", second_highest_digit);
        number.push_str(&second_highest_digit.to_string());
        println!("number: {:?}", number);
        sum += number.parse::<i64>().expect("failed to parse number to i64");
    }
    sum
}

fn solve_puzzle2(input: &str) -> i64 {
    0
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
        const EXAMPLE_OUTPUT: i64 = 357;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i64 = 6;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
