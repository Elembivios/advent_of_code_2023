use crate::utils::{point::{Coord, Point, Grid}, wait_user_input};
use anyhow::{Result, Error, anyhow};
enum Space {
    Digit(char),
    Symbol(char),
    Empty
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            c => {
                if c.is_ascii_digit() {
                    Self::Digit(c)
                } else {
                    Self::Symbol(c)
                }
            }
        }
    }
}

type P = Point<usize, Space>;
type C = Coord<usize>;

pub struct GearRatios {
    engine_schematics: Grid<P>
}

impl crate::Advent for GearRatios {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let engine_schematic: Vec<Vec<P>> = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Point::new(x, y, Space::from(c))
                    }).collect()
            }).collect();
        
        let engine_schematics = Grid::new(engine_schematic);
        Self { engine_schematics }
    }

    fn part_01(&self) -> String {
        1.to_string()
    }

    fn part_02(&self) -> String {
        2.to_string()
    }
}


