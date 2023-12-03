#[derive(Debug, PartialEq, Eq)]
pub struct BBox {
    pub line: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug)]
pub struct PaddedSchematic(Vec<String>);

impl BBox {
    pub fn new(line: usize, left: usize, right: usize) -> BBox{
        BBox { line, left, right }
    }

    pub fn len(&self) -> usize {
        self.right - self.left
    }

    pub fn previous_char(&self) -> BBox {
        BBox::new(self.line, self.left - 1, self.left)
    }

    pub fn next_char(&self) -> BBox {
        BBox::new(self.line, self.right, self.right + 1)
    }

    pub fn chars_above(&self) -> BBox {
        BBox::new(self.line - 1, self.left - 1, self.right + 1)
    }

    pub fn chars_below(&self) -> BBox {
        BBox::new(self.line + 1, self.left - 1, self.right + 1)
    }

    pub fn is_surrounded_by_symbol(&self, schematic: &PaddedSchematic) -> bool {
        let mut surrounding_chars: String = String::with_capacity(self.len() * 2 + 6);
        let mut is_surrounded: bool = false;

        surrounding_chars.extend(schematic.chars_at(self.previous_char()));
        surrounding_chars.extend(schematic.chars_at(self.next_char()));

        if self.line > 0 {
            surrounding_chars.extend(schematic.chars_at(self.chars_above()));
        }
        if self.line < schematic.0.len() - 1 {
            surrounding_chars.extend(schematic.chars_at(self.chars_below()));
        }

        for character in surrounding_chars.chars() {
            if PaddedSchematic::is_symbol(&character) {
                is_surrounded = true;
            };
        }

        is_surrounded
    }
}

impl From<&str> for PaddedSchematic {
    fn from(original_schematic: &str) -> Self {
        PaddedSchematic(
            original_schematic
                .lines()
                .into_iter()
                .map(|line| { format!(".{line}.") })
                .collect()
        )
    }
}

impl PaddedSchematic {
    /// Get a Vec of bboxes describing the 
    /// positions of the numbers in
    /// the schematic
    pub fn number_bboxes(&self) -> Vec<BBox> {
        let mut bboxes: Vec<BBox> = vec![];
        let mut bbox = BBox::new(0, 0, 0);
        let mut is_reading: bool = false;

        for (i, line) in self.0.iter().enumerate() {
            for (j, character) in line.chars().enumerate() {

                if character.is_digit(10) {
                    if !is_reading {
                        // start reading a number
                        is_reading = true;
                        bbox.line = i;
                        bbox.left = j;
                    }
                } else {
                    if is_reading {
                        // stop reading a number
                        is_reading = false;
                        bbox.right = j;
                        bboxes.push(bbox);
                        bbox = BBox::new(0, 0, 0);
                    }
                }
            }
        }
        bboxes
    }

    pub fn is_symbol(c: &char) -> bool {
        (*c != '.') && !c.is_digit(10)
    }

    /// Get all characters in line i
    /// from left to right coordinate
    pub fn chars_at(&self, bbox: BBox) -> Vec<char> {
        let mut result: Vec<char> = Vec::with_capacity(bbox.len());
        self.0
            .iter()
            .nth(bbox.line)
            .expect(format!("schamtic should have {} lines", bbox.line).as_str())
            .chars()
            .into_iter()
            .enumerate()
            .for_each(|(j, character)| { 
                if j >= bbox.left && j < bbox.right {
                    result.push(character);
                }
             });
        result
    }

    pub fn number_at(&self, bbox: BBox) -> i32 {
        self 
            .chars_at(bbox)
            .into_iter()
            .collect::<String>() 
            .parse::<i32>()
            .expect("bboxed chars should be castable to i32")
    }

    pub fn part_numbers(&self) -> Vec<i32> {
        self 
            .number_bboxes()
            .into_iter()
            .filter(|bbox| { bbox.is_surrounded_by_symbol(&self) })
            .map(|bbox| { self.number_at(bbox) })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_is_surrounded_by_symbol() {
        let schematic = PaddedSchematic::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        );
        let bboxes = vec![
            BBox::new(0, 1, 4),
            BBox::new(0, 6, 9),
            BBox::new(2, 7, 10),
        ];
        let is_surrounded_flags = vec![true, false, true];
        let results: Vec<bool> = bboxes
            .into_iter()
            .map(|bbox| { bbox.is_surrounded_by_symbol(&schematic) }) 
            .collect();
        
        for i in 0..2 {
            assert_eq!(results[i], is_surrounded_flags[i])
        }
    }

    #[test]
    fn test_schematic_number_bboxes() {
        let schematic = PaddedSchematic::from("467..114..
...*......
..35..633.
"
        );
        let bboxes = vec![
            BBox::new(0, 1, 4),
            BBox::new(0, 6, 9),
            BBox::new(2, 3, 5),
            BBox::new(2, 7, 10)
        ];
        let result = schematic.number_bboxes();
        assert_eq!(result, bboxes);
    }

    #[test]
    fn test_schematic_is_symbol() {
        let characters = vec!['.', '#', '2', '*'];
        let symbol_flags = vec![false, true, false, true];
        for i in 0..2 {
            let result = PaddedSchematic::is_symbol(&characters[i]);
            assert_eq!(result, symbol_flags[i]);
        }
    }

    #[test]
    fn test_schematic_get_chars() {
        let schematic = PaddedSchematic::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        );
        let bboxes = vec![
            BBox::new(0, 1, 4),
            BBox::new(1, 3, 6)
        ];
        let strings_to_be_read = vec![
            vec!['4', '6', '7'],
            vec!['.', '*', '.']
        ];
        for (i, bbox) in bboxes.into_iter().enumerate() {
            let result = schematic.chars_at(bbox);
            assert_eq!(result, strings_to_be_read[i]);
        }
    }

    #[test]
    fn schematic_part_numbers() {
        let schematic = PaddedSchematic::from("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        );
        let output = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let result = schematic.part_numbers();
        assert_eq!(result, output);
    }
}