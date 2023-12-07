use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub struct CamelCards {
    hands: Vec<Hand>
}

impl crate::Advent for CamelCards {
    fn new(data: &str) -> Self {
        let hands = data.lines().map(|l| {
            let (cards_s, bid_s) = l.split_once(" ").unwrap();
            let cards: [Card; 5] = cards_s.chars().map(|c| Card::new(c)).collect::<Vec<Card>>().try_into().unwrap();
            let bid = bid_s.parse().unwrap();
            Hand::new(cards, bid)
        }).collect();
        println!("Hands: {:?}", hands);
        Self { hands }
    }

    fn part_01(&self) -> String {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| {
            let first_unequal = a.cards.iter().zip(&b.cards).map(|(a, b)| {
                a.value.cmp(&b.value)          
            }).find(|c| !c.is_eq());
            if let Some(unequal) = first_unequal {
                unequal
            } else {
                Ordering::Equal
            }
        });
        
        println!("Sorted hands: {:?}", hands);

        1.to_string()       
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    equal_cards: Vec<u8>
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut equals: HashMap<char, u8> = HashMap::new();
        for card in &cards {
            *equals.entry(card.label).or_default() += 1;
        }
        Self {
            cards, bid,
            equal_cards: equals.into_values().collect::<Vec<u8>>()
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        
    }
}



#[derive(Debug, Clone)]
struct Card {
    label: char,
    value: u8
}

impl Card {
    fn new(label: char) -> Self {
        let value = Card::label_to_value(label);
        Self { label, value}
    }

    fn label_to_value(label: char) -> u8 {
        if label.is_ascii_digit() {
            label.to_digit(10).unwrap() as u8
        } else {
            match label {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Invalid card label {}", label)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hex_to_ascii() {
        let a = 'A';
        let b = '9';

        assert!(a > b);
    }
}