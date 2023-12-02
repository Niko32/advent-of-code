use std::{fmt::Display, ops::Deref, panic::UnwindSafe};

#[derive(Debug, PartialEq, Eq)]
pub struct Sample {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
pub struct ParseSampleError;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    samples: Vec<Sample>
}

#[derive(Debug)]
pub struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
pub struct Games(Vec<Game>);


impl Display for ParseSampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse a sample of colored cubes.")
    }
}

impl Display for ParseGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse a single game of colored cubes.")
    }
}

impl TryFrom<&str> for Sample {
    type Error = ParseSampleError;

    fn try_from(s: &str) -> Result<Self, ParseSampleError> {
        let substrings: Vec<&str> = s
            .split(",")
            .map(|x| { x.trim() })
            .collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for substring in substrings {
            let (amount, color) = substring
                .split(" ")
                .
                .unwrap();
            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => return Err(ParseSampleError)
            }
        }

        Ok(Sample{ red, green, blue })
    }
}

impl Sample {
    pub fn new(red: u32, green: u32, blue: u32) -> Sample {
        Sample{ red, green, blue }
    }

    /// Returns true if the given sample is a subset
    /// of self
    pub fn contains(&self, sample: Sample) -> bool {
        (self.red >= sample.red) && (self.green >= sample.green) && (self.blue >= sample.blue) 
    }

    /// Computes the union of multiple samples
    /// i.e. the maximum of each color for a set
    /// of sample
    pub fn union(samples: &Vec<Sample>) -> Sample {
        let mut reds: Vec<u32> = Vec::with_capacity(samples.len());
        let mut greens: Vec<u32> = Vec::with_capacity(samples.len());
        let mut blues: Vec<u32> = Vec::with_capacity(samples.len());

        for sample in samples {
            reds.push(sample.red);
            greens.push(sample.green);
            blues.push(sample.blue);
        }

        let red = reds.into_iter().max().unwrap();
        let green = greens.into_iter().max().unwrap();
        let blue = blues.into_iter().max().unwrap();

        Sample{ red, green, blue }
    }
}

impl TryFrom<&str> for Game {
    type Error = ParseGameError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let id = s
            .split(":")
            .collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .expect("game id substrings should not be empty")
            .deref()
            .parse::<u32>()
            .expect("game id substring should be castable to u32");

        let samples: Vec<Sample> = s[8..]
            .split(";")
            .map(|x| { x.trim() })
            .map(|x| { Sample::try_from(x).unwrap() })
            .collect::<Vec<Sample>>();

        Ok(Game { id, samples })
    }
}

impl From<&str> for Games {
    fn from(s: &str) -> Self {
        let games = s.lines()
            .map(|line| { Game::try_from(line).unwrap() })
            .collect::<Vec<Game>>();
        Games(games)
    }
}

impl Games {
    /// Returns a vector of games that would be possible with
    /// a given set of cubes in the entire bag
    pub fn possible_games_with(&self, sample: Sample) -> Vec<&Game> {
        self.0.iter()
            .filter(|game| {
                sample.contains(Sample::union(&game.samples))
            })
            .collect() 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_from_string() {
        const INPUT: &str = "8 green, 4 red, 4 blue";
        const OUTPUT: Sample = Sample{ red: 4, green: 8, blue: 4};
        let result = Sample::try_from(INPUT).unwrap();
        assert_eq!(result, OUTPUT)
    }

    #[test]
    fn test_game_from_string() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let output: Game = Game{ 
            id: 1, 
            samples: vec![
                Sample{ red: 4, green: 0, blue: 3 },
                Sample{ red: 1, green: 2, blue: 6 },
                Sample{ red: 0, green: 2, blue: 0 }
            ] 
        };
        let result = Game::try_from(INPUT).unwrap();
        assert_eq!(result, output)
    }
}