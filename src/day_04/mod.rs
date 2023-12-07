use std::collections::{HashSet, HashMap};

pub struct Scratchcards {
    cards: Vec<[HashSet<u8>;2]>
}

fn split_numbers(s: &str) -> HashSet<u8> {
    s.split(" ").filter(|s| *s != " " && !s.is_empty()).map(|s| s.parse().unwrap()).collect()
}

impl crate::Advent for Scratchcards {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let cards = data.lines().map(|l| {
            let (_, rhs) = l.split_once(": ").unwrap();
            let (winning_s, yours_s) = rhs.split_once(" | ").unwrap();
            let winning = split_numbers(winning_s);
            let your_numbers = split_numbers(yours_s);            
            [winning, your_numbers]
        }).collect();
        Self { cards }
    }

    fn part_01(&self) -> String {
        let cards = self.cards.clone();
        let points: Vec<u32> = cards.iter().map(|c| {
            c[0].intersection(&c[1]).collect::<Vec<_>>()
        }).map(|i| {
            if i.len() > 0 {
                2u32.pow(i.len() as u32 - 1)
            } else {
                0 
            }                
        }).collect();
        points.iter().sum::<u32>().to_string()    
    }

    fn part_02(&self) -> String {
        let cards = self.cards.clone();
        let points: Vec<(u32, u32)> = cards.iter().map(|c| {
            c[0].intersection(&c[1]).collect::<Vec<_>>()
        }).map(|i| {
            if i.len() > 0 {
                (2u32.pow(i.len() as u32 - 1), i.len() as u32)
            } else {
                (0, 0)
            }                
        }).collect();
        let mut copies: HashMap<usize, usize> = HashMap::from_iter((0..points.len()).map(|i| (i, 1)));
        for (i, (_point, winnings)) in points.into_iter().enumerate() {
            let nr = *copies.get(&i).unwrap();

            if winnings > 0 {
                for j in i+1..i+winnings as usize + 1 {
                    *copies.get_mut(&j).unwrap() += nr;
                }
            }
        }
        
        copies.values().sum::<usize>().to_string()
    }
}