use std::{cmp::Ordering, collections::HashMap};
use std::fmt;

pub struct CamelCards {
    hands_01: Vec<Hand>,
    hands_02: Vec<Hand>
}

impl crate::Advent for CamelCards {
    fn new(data: &str) -> Self {
        let hands_01 = data.lines().map(|l| {
            let (cards_s, bid_s) = l.split_once(" ").unwrap();
            let cards: [Card; 5] = cards_s.chars().map(|c| Card::new(c)).collect::<Vec<Card>>().try_into().unwrap();
            let bid = bid_s.parse().unwrap();
            Hand::new(cards, bid)
        }).collect();
        let hands_02 = data.lines().map(|l| {
            let (cards_s, bid_s) = l.split_once(" ").unwrap();
            let cards: [Card; 5] = cards_s.chars().map(|c| Card::new_w_joker(c)).collect::<Vec<Card>>().try_into().unwrap();
            let bid = bid_s.parse().unwrap();
            Hand::new_w_joker(cards, bid)
        }).collect();
        Self { hands_01, hands_02 }
    }

    fn part_01(&self) -> String {
        let mut hands = self.hands_01.clone();
        hands.sort_by(|a, b| a.alt_cmp(&b));
        let mut sum: u32 = 0;
        for (i, hand) in hands.into_iter().enumerate() {
            sum += (i as u32 + 1) * hand.bid
        }
        sum.to_string()    
    }

    fn part_02(&self) -> String {
        let mut hands = self.hands_02.clone();
        hands.sort_by(|a, b| a.alt_cmp(&b));
        let mut sum: u32 = 0;
        for (i, hand) in hands.into_iter().enumerate() {
            sum += (i as u32 + 1) * hand.bid
        }
        sum.to_string()    
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    cards_count: Vec<(Card, u8)>
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut equals: HashMap<_, _> = HashMap::new();
        for card in &cards {
            *equals.entry(card.clone()).or_default() += 1;
        }
        let mut equals_sorted: Vec<(Card, u8)> = equals.into_iter().collect();
        equals_sorted.sort_by(|a, b| {
            a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse()  
        });

        Self {
            cards, bid,
            cards_count: equals_sorted
        }
    }

    fn new_w_joker(cards: [Card; 5], bid: u32) -> Self {
        let mut equals: HashMap<_, _> = HashMap::new();
        for card in cards.iter().filter(|c| c.label != 'J') {
            *equals.entry(card.clone()).or_default() += 1;
        }
        
        let mut equals_sorted: Vec<(Card, u8)> = equals.into_iter().collect();
        equals_sorted.sort_by(|a, b| {
            a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse()  
        });
        let max_equals = equals_sorted.get_mut(0);
        if let Some(max_equals) = max_equals {
            max_equals.1 += cards.iter().filter(|c| c.label == 'J').count() as u8; 
        } else {
            equals_sorted.push((Card::new('A'), 5u8));
        }
        
        Self {
            cards, bid,
            cards_count: equals_sorted
        } 
    }

    fn alt_cmp(&self, other: &Self) -> Ordering {
        let cards_count_it = self.cards_count.iter().zip(&other.cards_count);        
        let count_ord = cards_count_it.map(|(a, b)| {
            a.1.cmp(&b.1)
        }).find(|c| !c.is_eq());
        if let Some(ord) = count_ord {
            ord
        } else {
            let cards_ord = self.cards
                .iter()
                .zip(&other.cards)
                .map(|(a, b)| a.cmp(&b))
                .find(|c| !c.is_eq());
            if let Some(ord) = cards_ord {
                ord
            } else {
                Ordering::Equal
            }                
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.cards_count.iter().zip(&other.cards_count).map(|(a, b)| {
            a.1.cmp(&b.1).then(a.0.cmp(&b.0))
        }).find(|c| !c.is_eq());
        if let Some(ord) = ord {
            ord
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Clone, Hash)]
struct Card {
    label: char,
    value: u8
}

impl Card {
    fn new(label: char) -> Self {
        let value = Card::label_to_value(label);
        Self { label, value}
    }

    fn new_w_joker(label: char) -> Self {
        let value = if label == 'J' {
            1
        } else {
            Card::label_to_value(label)
        };
        Self { label, value }
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Card {}

#[cfg(test)]
mod tests {
    #[test]
    fn hex_to_ascii() {
        let a = 'A';
        let b = '9';

        assert!(a > b);
    }
}