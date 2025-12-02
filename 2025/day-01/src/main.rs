mod structs;

pub const EXAMPLE_INPUT: &str = include_str!("../data/example_input.txt");

fn solve_puzzle1(input: &str) -> i64 {
    let rotations: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let mut dial = 50;
    let mut zeros = 0;
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

// pub fn solve_puzzle2(input: &str) -> i64 {
//     let race = Race::from(input);
//     race.n_winning_strategies() as i64
// }

fn main() {
    const INPUT: &str = include_str!("../data/input1.txt");

    let solution1 = solve_puzzle1(INPUT);
    println!("Solution to puzzle one: {}", solution1);

    // let solution2 = solve_puzzle2(INPUT);
    // println!("Solution to puzzle two: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input1() {
        const EXAMPLE_OUTPUT: i64 = 3;
        let result = solve_puzzle1(EXAMPLE_INPUT);
        assert_eq!(result, EXAMPLE_OUTPUT);
    }

    // #[test]
    // fn test_example_input2() {
    //     const EXAMPLE_OUTPUT: i64 = 71503;
    //     let result = solve_puzzle2(EXAMPLE_INPUT);
    //     assert_eq!(result, EXAMPLE_OUTPUT);
    // }
}
