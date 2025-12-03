mod structs;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

fn parse_ranges(s: &str) -> Vec<(&str, &str)> {
    s.trim_end()
        .split(",")
        .map(|substring| {
            substring.split_once("-")
                .expect("range has more than one - delimiter")
        })
        .collect()
}

fn is_invalid(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false
    }

    let (a, b) = s.split_at((s.len() / 2) as usize);
    return a == b
}

fn is_invalid2(s: &str) -> bool {
    let mut substring = String::new();
    'chars: for c in s.chars().take(s.chars().count() / 2) {
        substring.push(c);

        if s.len() % substring.len() != 0 {
            continue
        }

        for i in (0..s.len()).step_by(substring.len()) {
            if substring != &s[i..i+substring.len()] {
                continue 'chars
            }
        }
        return true
    }
    return false
}

fn solve_puzzle1(input: &str) -> i64 {
    let mut sum = 0;

    for (start, end) in parse_ranges(input) {
        println!("{:?}, {:?}", start, end);
        let start_int: usize = start.parse().expect("failed to parse start to int");
        let end_int: usize = end.parse().expect("failed to parse end to int");

        for i in start_int..=end_int {
            if is_invalid(&i.to_string()) {
                sum += i
            }
        }
    }

    return sum as i64
}

fn solve_puzzle2(input: &str) -> i64 {
    let mut sum = 0;

    for (start, end) in parse_ranges(input) {
        println!("{:?}, {:?}", start, end);
        let start_int: usize = start.parse().expect("failed to parse start to int");
        let end_int: usize = end.parse().expect("failed to parse end to int");

        for i in start_int..=end_int {
            if is_invalid2(&i.to_string()) {
                sum += i;
            }
        }
    }

    return sum as i64
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
        const EXAMPLE_OUTPUT: i64 = 1227775554;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    #[test]
    fn test_is_invalid2() {
        assert!(is_invalid2("12341234"));
        assert!(is_invalid2("123123123"));
        assert!(is_invalid2("1212121212"));
        assert!(is_invalid2("1111111"));
    }

    #[test]
    fn test_example_input2() {
        const EXAMPLE_OUTPUT: i64 = 4174379265;
        let result = solve_puzzle2(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }
}
