use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MapRange {
    source_start: u64,
    target_start: u64,
    len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    vec: Vec<MapRange>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut split = if value.contains("\r\n") {
            value.split("\r\n\r\n")
        } else {
            value.split("\n\n")
        };
        let seed_str = split.next().expect("split should have one element");
        let maps = split.map(Map::from).collect::<Vec<Map>>();

        let seeds: Vec<u64> = seed_str
            .split(':')
            .nth(1)
            .expect("seed str to have two elements")
            .split_ascii_whitespace()
            .map(|substr| substr.parse::<u64>().expect("seed ids should be parsable"))
            .collect::<Vec<u64>>();
        Almanac { seeds, maps }
    }
}

impl Almanac {
    pub fn nearest_seed_location(&self) -> u64 {
        self.seeds
            .par_iter()
            .map(|seed| self.seed_location(*seed))
            .min()
            .unwrap()
    }

    pub fn seed_location(&self, seed_id: u64) -> u64 {
        let mut map_target = seed_id;
        for map in &self.maps {
            map_target = map.convert(map_target);
        }
        map_target
    }

    pub fn from_ranges(value: &str) -> Self {
        let mut split = if value.contains("\r\n") {
            value.split("\r\n\r\n")
        } else {
            value.split("\n\n")
        };
        let seed_str = split.next().expect("split should have one element");
        let maps = split.map(Map::from).collect::<Vec<Map>>();

        let seeds: Vec<u64> = Self::seeds_from_ranges(seed_str);
        Almanac { seeds, maps }
    }

    fn seeds_from_ranges(value: &str) -> Vec<u64> {
        value
            .split(':')
            .nth(1)
            .expect("seed str to have two elements")
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0]
                    .parse::<u64>()
                    .expect("seed start should be parsable");
                let len = chunk[1]
                    .parse::<u64>()
                    .expect("seed length should be parsable");
                start..start + len
            })
            .collect::<Vec<u64>>()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            vec: value
                .lines()
                .skip(1)
                .map(MapRange::from)
                .collect::<Vec<MapRange>>(),
        }
    }
}

impl Map {
    pub fn convert(&self, id: u64) -> u64 {
        for range in &self.vec {
            if (range.source_start..=range.source_start + range.len as u64).contains(&(id)) {
                return id + (range.target_start - range.source_start);
            }
        }
        id
    }
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let mut split = value.split_ascii_whitespace();
        let target_start = split.next().unwrap().parse().unwrap();
        let source_start = split.next().unwrap().parse().unwrap();
        let len = split.next().unwrap().parse().unwrap();

        MapRange {
            source_start,
            target_start,
            len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_convert() {
        let map = Map {
            vec: vec![
                MapRange {
                    source_start: 98,
                    target_start: 50,
                    len: 2,
                },
                MapRange {
                    source_start: 50,
                    target_start: 52,
                    len: 48,
                },
            ],
        };

        let inputs = vec![99, 10];
        let outputs = vec![51, 10];
        let results: Vec<u64> = inputs.into_iter().map(|input| map.convert(input)).collect();
        assert_eq!(results, outputs);
    }

    #[test]
    fn test_almanac_seeds_from_range() {
        let input = "seeds: 79 14 55 13";
        let output: Vec<u64> = vec![
            79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61, 62,
            63, 64, 65, 66, 67,
        ];
        let result = Almanac::seeds_from_ranges(input);
        assert_eq!(result, output);
    }
}
