use deref_derive::{Deref, DerefMut};
use std::cmp::Ordering;

#[derive(Deref, DerefMut, Debug, Eq, PartialEq)]
pub struct Hands(Vec<Hand>);

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: [char; 5],
    bid: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CardType {
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<&str> for Hands {
    fn from(value: &str) -> Self {
        Self(value.lines().map(Hand::from).collect::<Vec<Hand>>())
    }
}

impl Hands {
    pub fn total_winnings(&mut self) -> u64 {
        self.sort();
        self.iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u64 * hand.bid)
            .sum()
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut split = value.split_ascii_whitespace();
        let cards: [char; 5] = split
            .next()
            .expect("Hand string to have at least one element")
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .expect("Cards string should be castable to [char;5]");
        let bid: u64 = split
            .next()
            .expect("Hand string to have at least two elements")
            .parse::<u64>()
            .expect("Bid string to be parsable to u64");
        Hand { cards, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let own_card_type = self.card_type();
        let other_card_type = other.card_type();

        if own_card_type != other_card_type {
            own_card_type.cmp(&other_card_type)
        } else {
            for (i, c) in self.cards.into_iter().enumerate() {
                let c = CardType::from(c);
                let other_c = CardType::from(other.cards[i]);
                if c != other_c {
                    return c.cmp(&other_c);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn card_type(&self) -> HandType {
        let mut amounts_of_matches: [u64; 5] = [0, 0, 0, 0, 0];
        for c in &self.cards {
            for (i, d) in self.cards.iter().enumerate() {
                if c == d {
                    amounts_of_matches[i] += 1;
                }
            }
        }

        match amounts_of_matches {
            [5, 5, 5, 5, 5] => HandType::FiveOfAKind,
            _ if amounts_of_matches.contains(&4) => HandType::FourOfAKind,
            _ if amounts_of_matches.contains(&3) && amounts_of_matches.contains(&2) => {
                HandType::FullHouse
            }
            _ if amounts_of_matches.contains(&3) => HandType::ThreeOfAKind,
            _ if amounts_of_matches.iter().sum::<u64>() == 9_u64 => HandType::TwoPair,
            _ if amounts_of_matches.iter().sum::<u64>() == 7_u64 => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Undefined match arm: {:?}", amounts_of_matches),
        }
    }
}

impl From<char> for CardType {
    fn from(value: char) -> Self {
        match value {
            '2' => CardType::D2,
            '3' => CardType::D3,
            '4' => CardType::D4,
            '5' => CardType::D5,
            '6' => CardType::D6,
            '7' => CardType::D7,
            '8' => CardType::D8,
            '9' => CardType::D9,
            'T' => CardType::T,
            'J' => CardType::J,
            'Q' => CardType::Q,
            'K' => CardType::K,
            'A' => CardType::A,
            _ => panic!("CardType coult not be constructed from char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hands_sort() {
        let hands = Hands(vec![
            Hand {
                cards: ['3', '2', 'T', '3', 'K'],
                bid: 765,
            },
            Hand {
                cards: ['T', '5', '5', 'J', '5'],
                bid: 684,
            },
            Hand {
                cards: ['K', 'K', '6', '7', '7'],
                bid: 28,
            },
            Hand {
                cards: ['K', 'T', 'J', 'J', 'T'],
                bid: 220,
            },
            Hand {
                cards: ['Q', 'Q', 'Q', 'J', 'A'],
                bid: 483,
            },
        ]);

        let output = Hands(vec![
            Hand {
                cards: ['3', '2', 'T', '3', 'K'],
                bid: 765,
            },
            Hand {
                cards: ['K', 'T', 'J', 'J', 'T'],
                bid: 220,
            },
            Hand {
                cards: ['K', 'K', '6', '7', '7'],
                bid: 28,
            },
            Hand {
                cards: ['T', '5', '5', 'J', '5'],
                bid: 684,
            },
            Hand {
                cards: ['Q', 'Q', 'Q', 'J', 'A'],
                bid: 483,
            },
        ]);

        let mut result = hands;
        result.sort();
        assert_eq!(result, output);
    }
}
