#[derive(Debug, PartialEq, Eq)]
pub struct MapRange {
    source_start: u32,
    target_start: u32,
    len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    vec: Vec<MapRange>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Almanac {
    seeds: Vec<u32>,
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

        dbg!(seed_str);
        let seeds: Vec<u32> = seed_str
            .split(':')
            .nth(1)
            .expect("seed str to have two elements")
            .split_ascii_whitespace()
            .map(|substr| substr.parse::<u32>().expect("seed ids should be parsable"))
            .collect::<Vec<u32>>();
        Almanac { seeds, maps }
    }
}

impl Almanac {
    pub fn nearest_seed_location(&self) -> u32 {
        self.seeds
            .iter()
            .map(|seed| self.seed_location(*seed))
            .min()
            .unwrap()
    }

    pub fn seed_location(&self, seed_id: u32) -> u32 {
        let mut map_target = seed_id;
        for map in &self.maps {
            map_target = map.convert(map_target);
        }
        map_target
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
    pub fn convert(&self, id: u32) -> u32 {
        for range in &self.vec {
            if (range.source_start as u64..=range.source_start as u64 + range.len as u64)
                .contains(&(id as u64))
            {
                return (id as i64 + (range.target_start as i64 - range.source_start as i64))
                    as u32;
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
        let results: Vec<u32> = inputs.into_iter().map(|input| map.convert(input)).collect();
        assert_eq!(results, outputs);
    }
}
