#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub id: usize,
    pub own_numbers: Vec<i32>,
    pub winning_numbers: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CardPile {
    vec: Vec<(Card, u32)>,
}

impl From<&str> for Card {
    fn from(input: &str) -> Card {
        let split = input
            .split(':')
            .map(|substr| substr.trim())
            .collect::<Vec<&str>>();
        let [id_str, numbers_str] = <[&str; 2]>::try_from(split)
            .expect("first split when parsing a card should contain two values");
        let split = numbers_str
            .split('|')
            .map(|substr| substr.trim())
            .collect::<Vec<&str>>();
        let [own_numbers_str, winning_numbers_str] = <[&str; 2]>::try_from(split)
            .expect("second split when parsing a card should contain two values");
        let id: usize = id_str
            .split_ascii_whitespace()
            .nth(1)
            .expect("game id split should contain two values when parsing a card")
            .parse::<usize>()
            .expect("game id should be castable to i32");
        let own_numbers: Vec<i32> = own_numbers_str
            .split_ascii_whitespace()
            .map(|substr| {
                substr
                    .parse::<i32>()
                    .expect("contents of a numbers substring should be castable to i32")
            })
            .collect();
        let winning_numbers: Vec<i32> = winning_numbers_str
            .split_ascii_whitespace()
            .map(|substr| {
                substr
                    .parse::<i32>()
                    .expect("contents of a numbers substring should be castable to 32")
            })
            .collect();
        Card {
            id,
            own_numbers,
            winning_numbers,
        }
    }
}

impl Card {
    pub fn count_points(&self) -> u32 {
        let exponent = self.count_wins();
        if exponent == 0 {
            0
        } else {
            2_u32.pow(exponent as u32 - 1)
        }
    }

    pub fn count_wins(&self) -> usize {
        self.own_numbers
            .iter()
            .map(|own_number| self.winning_numbers.as_slice().contains(own_number))
            .filter(|boolean| *boolean)
            .count()
    }
}

impl From<&str> for CardPile {
    fn from(input: &str) -> CardPile {
        CardPile {
            vec: input.lines().map(|line| (Card::from(line), 1)).collect(),
        }
    }
}

impl CardPile {
    pub fn count_points(&self) -> u32 {
        self.vec
            .iter()
            .map(|card| card.0.count_points() * card.1)
            .sum()
    }

    pub fn copy_card(&mut self, game_id: usize, n_times: u32) {
        let card_to_copy: &mut (Card, u32) = self
            .vec
            .get_mut(game_id - 1)
            .expect("given id should be contained in card pile");
        card_to_copy.1 += n_times;
    }

    pub fn win_more_cards(&mut self) {
        for i in 0..self.vec.len() - 1 {
            let card = self.vec.get(i).expect("vector should be indexable by i");
            let wins = card.0.count_wins();
            let card_id = card.0.id;
            let n_copies = card.1;
            for j in 1..wins + 1 {
                self.copy_card(card_id + j, n_copies);
            }
        }
    }

    pub fn count_copies(&self) -> Vec<u32> {
        self.vec.iter().map(|card| card.1).collect()
    }

    pub fn count_cards(&self) -> u32 {
        self.count_copies().iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_pile_count_copies() {
        const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let output = vec![1, 2, 4, 8, 14, 1];

        let mut card_pile = CardPile::from(EXAMPLE_INPUT);
        card_pile.win_more_cards();
        let result = card_pile.count_copies();
        assert_eq!(result, output);
    }
}
