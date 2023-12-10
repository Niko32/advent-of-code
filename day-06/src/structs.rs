pub struct Races {
    pub vec: Vec<Race>,
}

pub struct Race {
    pub allowed_time: u64,
    pub record_distance: u64,
}

impl From<&str> for Races {
    fn from(value: &str) -> Self {
        let mut split = value.lines();
        let time_str = split.next().expect("Split should have one line for time");
        let distance_str = split
            .next()
            .expect("Split should have a second line for distance");

        let times = time_str
            .split(':')
            .nth(1)
            .expect("Time string should contain times after colon")
            .split_ascii_whitespace()
            .map(|time_str| {
                time_str
                    .parse::<u64>()
                    .expect("Time substrings should be parsable as u64")
            });
        let distances = distance_str
            .split(':')
            .nth(1)
            .expect("Distance string should contain times after colon")
            .split_ascii_whitespace()
            .map(|s| {
                s.parse::<u64>()
                    .expect("Distance substrings should be parsable as u64")
            });

        Races {
            vec: times
                .zip(distances)
                .into_iter()
                .map(|tuple| Race {
                    allowed_time: tuple.0,
                    record_distance: tuple.1,
                })
                .collect(),
        }
    }
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        let mut split = value.lines();
        let time_str = split.next().expect("Split should have one line for time");
        let distance_str = split
            .next()
            .expect("Split should have a second line for distance");

        let time = time_str
            .split(':')
            .nth(1)
            .expect("Time string should contain times after colon")
            .replace(' ', "")
            .parse::<u64>()
            .expect("Time should be parsable as u64");

        let distance = distance_str
            .split(':')
            .nth(1)
            .expect("Distance string should contain times after colon")
            .replace(' ', "")
            .parse::<u64>()
            .expect("Distance string should be parsable as u64");

        Race {
            allowed_time: time,
            record_distance: distance,
        }
    }
}

impl Race {
    pub fn strategies(&self) -> Vec<u64> {
        let mut strategies = Vec::with_capacity(self.allowed_time as usize + 1);
        for charge_time in 0..=self.allowed_time {
            let distance = (self.allowed_time - charge_time) * charge_time;
            strategies.push(distance);
        }
        strategies
    }

    pub fn n_winning_strategies(&self) -> usize {
        self.strategies()
            .into_iter()
            .filter(|&strategy| strategy > self.record_distance)
            .count()
    }
}
