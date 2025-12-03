mod structs;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

fn solve_puzzle1(input: &str) -> i32 {
    let rotations: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let mut dial: i32 = 50;
    let mut zeros: i32 = 0;
    for r in &rotations {
        let (direction, distance) = r.split_at(1);

        let steps: i32 = distance.parse()
            .expect("failed to parse distance as i32");

        match direction {
            "L" => {
                dial = ((dial + 100) - steps) % 100
            }
            "R" => {
                dial = (dial + steps) % 100
            }
            _ => panic!("invalid direction")
        }

        if dial == 0 {
            zeros += 1
        }
    }

    return zeros
}

fn solve_puzzle2(input: &str) -> i32 {
    let rotations: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let mut dial: i32 = 50;
    let mut zeros: i32 = 0;
    let mut steps: i32 = 0;
    let mut direction: &str = "";
    let mut distance: &str = "";
    for r in &rotations {
        (direction, distance) = r.split_at(1);

        steps = distance.parse()
            .expect("failed to parse distance as i32");
        println!("{:?} Steps in {:?} direction, dial: {:?}, zeros: {:?}", steps, direction, dial, zeros);

        while steps > 100 {
            steps -= 100;
            zeros += 1;
        }
        
        match direction {
            "L" => {
                if (dial != 0 && dial < steps) {
                    zeros += 1;
                }
                dial = ((dial + 100) - steps) % 100
            }
            "R" => {
                if dial + steps > 100 {
                    zeros += 1;
                }
                dial = (dial + steps) % 100
            }
            _ => panic!("invalid direction")
        }

        if dial == 0 {
            zeros += 1;
        }
    }
    println!("{:?} Steps in {:?} direction, dial: {:?}, zeros: {:?}", steps, direction, dial, zeros);

    return zeros
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
