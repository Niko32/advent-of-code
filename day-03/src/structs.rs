use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BBox {
    pub line: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug)]
pub struct PaddedSchematic(Vec<String>);

#[derive(Debug)]
pub struct Gear<'a> {
    schematic: &'a PaddedSchematic,
    pos: BBox,
    part_numbers: [i32; 2],
}

#[derive(Debug)]
pub struct GearCreationError(BBox);

impl Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BBox{{ line: {}, left: {}, right: {}}}", self.line, self.left, self.right)
    }
}

impl BBox {
    pub fn new(line: usize, left: usize, right: usize) -> BBox{
        BBox { line, left, right }
    }

    pub fn len(&self) -> usize {
        self.right - self.left
    }

    pub fn bbox_to_the_left(&self) -> BBox {
        BBox::new(self.line, self.left - 1, self.left)
    }

    pub fn bbox_to_the_right(&self) -> BBox {
        BBox::new(self.line, self.right, self.right + 1)
    }

    pub fn bbox_above(&self) -> BBox {
        BBox::new(self.line - 1, self.left - 1, self.right + 1)
    }

    pub fn bbox_below(&self) -> BBox {
        BBox::new(self.line + 1, self.left - 1, self.right + 1)
    }

    pub fn small_bbox_above(&self) -> BBox {
        BBox::new(self.line - 1, self.left, self.right)
    }

    pub fn small_bbox_below(&self) -> BBox {
        BBox::new(self.line + 1, self.left, self.right)
    }

    pub fn bbox_to_the_top_left(&self) -> BBox {
        BBox::new(self.line - 1, self.left - 1, self.right - 1)
    }

    pub fn bbox_to_the_top_right(&self) -> BBox {
        BBox::new(self.line - 1, self.left + 1, self.right + 1)
    }

    pub fn bbox_to_the_bottom_left(&self) -> BBox {
        BBox::new(self.line + 1, self.left - 1, self.right - 1)
    }

    pub fn bbox_to_the_bottom_right(&self) -> BBox {
        BBox::new(self.line + 1, self.left + 1, self.right + 1)
    }

    pub fn is_surrounded_by_symbol(&self, schematic: &PaddedSchematic) -> bool {
        let mut surrounding_chars: String = String::with_capacity(self.len() * 2 + 6);
        let mut is_surrounded: bool = false;

        surrounding_chars.extend(schematic.chars_at(&self.bbox_to_the_left()));
        surrounding_chars.extend(schematic.chars_at(&self.bbox_to_the_right()));

        if self.line > 0 {
            surrounding_chars.extend(schematic.chars_at(&self.bbox_above()));
        }
        if self.line < schematic.0.len() - 1 {
            surrounding_chars.extend(schematic.chars_at(&self.bbox_below()));
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
    pub fn chars_at(&self, bbox: &BBox) -> Vec<char> {
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
        let mut extended_bbox = bbox;
        while self.chars_at(&extended_bbox.bbox_to_the_left())[0].is_digit(10) {
            extended_bbox.left -= 1;
        }
        while self.chars_at(&extended_bbox.bbox_to_the_right())[0].is_digit(10) {
            extended_bbox.right += 1;
        }

        self 
            .chars_at(&extended_bbox)
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

    pub fn find_gears(&self) -> Vec<Gear> {
        let mut gears: Vec<Gear> = vec![];

        for (i, line) in self.0.iter().enumerate() {
            for (j, character) in line.chars().enumerate() {
                if character == '*' {
                    let pos = BBox::new(i, j, j + 1);
                    match Gear::new(self, pos) {
                        Ok(gear) => gears.push(gear),
                        Err(_) => (),
                    }
                }
            }
        }

        gears
    }
}

impl<'a> Gear<'a> {
    pub fn new(schematic: &'a PaddedSchematic, pos: BBox) -> Result<Gear<'a>, GearCreationError> {
        let mut part_numbers: Vec<i32> = vec![];

        fn try_adding_part_numbers(pos: BBox, part_numbers: &mut Vec<i32>, schematic: &PaddedSchematic) -> bool {
            if schematic.chars_at(&pos)[0].is_digit(10) {
                part_numbers.push(schematic.number_at(pos));
                true
            } else {
                false
            }
        }

        // add part numbers to the sides
        try_adding_part_numbers(pos.bbox_to_the_left(), &mut part_numbers, schematic);
        try_adding_part_numbers(pos.bbox_to_the_right(), &mut part_numbers, schematic);

        if !try_adding_part_numbers(pos.small_bbox_above(), &mut part_numbers, schematic) {
            // add part numbers in the upper corners
            try_adding_part_numbers(pos.bbox_to_the_top_left(), &mut part_numbers, schematic);
            try_adding_part_numbers(pos.bbox_to_the_top_right(), &mut part_numbers, schematic);
        }

        if !try_adding_part_numbers(pos.small_bbox_below(), &mut part_numbers, schematic) {
            // add part numbers in the bottom corners
            try_adding_part_numbers(pos.bbox_to_the_bottom_left(), &mut part_numbers, schematic);
            try_adding_part_numbers(pos.bbox_to_the_bottom_right(), &mut part_numbers, schematic);
        }

        if part_numbers.len() == 2 {
            let part_numbers: [i32; 2] = part_numbers.try_into()
                .expect("part numbers vector should have 2 elements");
            let gear: Gear<'a> = Gear { schematic, pos, part_numbers };
            Ok(gear)
        } else {
            Err(GearCreationError(pos))
        }
    }

    pub fn ratio(&self) -> i32 {
        self.part_numbers.into_iter().product()
    }
}

impl fmt::Display for GearCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No gear at position {}", self.0)
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
            let result = schematic.chars_at(&bbox);
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