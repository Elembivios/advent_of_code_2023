use crate::utils::point::{Coord, Point, Grid, DIRECTIONS};
use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Digit(d) => write!(f, "{}", d),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Empty => write!(f, ".")
        }
    }
}

type C = Coord<usize>;

pub struct GearRatios {
    engine_schematics: Grid<Space>
}

impl crate::Advent for GearRatios {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let engine_schematic: Vec<Vec<Space>> = data
            .lines()
            .map(|l| {
                l
                    .chars()
                    .map(|c| {
                        Space::from(c)
                    }).collect()
            }).collect();
        
        let engine_schematics = Grid::new(engine_schematic);
        Self { engine_schematics }
    }

    fn part_01(&self) -> String {
        let symbols: Vec<Point<usize, &Space>> = self.engine_schematics
            .iter_points()
            .filter(|p| {
                match p.value {
                    Space::Symbol(_s) => true,
                    _ => false
                }
        }).collect();
        let mut numbers: Vec<Point<usize, String>> = vec![];
        let mut start_coord: Option<C> = None;
        let mut number: String = String::new();
        let numbers_it = self.engine_schematics.iter_points();
        
        numbers_it.for_each(|p| {
            match p.value {
                Space::Digit(d) => {
                    if start_coord.is_none() {
                        start_coord = Some(p.coord);
                    }
                    number.push(*d);                    
                },
                _ => {
                    if number.len() > 0 {
                        let number_point: Point<usize, String> = Point::from_coord(start_coord.unwrap(), number.clone());
                        numbers.push(number_point);
                        number = String::new();
                        start_coord = None;
                    }
                }
            }
        });
        let mut result: u32 = 0;
        for symbol in symbols {
            let neighbour_coords: Vec<_> = DIRECTIONS
                .iter()
                .filter_map(|direction| {
                self.engine_schematics.get_neighbour(&symbol.coord, direction)
            }).collect();
            let neighbour_numbers: Vec<&Point<usize, String>> = numbers
                .iter().filter(|n| {
                    neighbour_coords.iter().any(|c| {
                        if n.coord.y == c.y && n.coord.x <= c.x && n.coord.x + n.value.len() - 1 >= c.x {
                            true
                        } else {
                            false
                        }
                    })                    
                }).collect();
            result += neighbour_numbers.iter().map(|p| p.value.parse::<u32>().unwrap()).sum::<u32>();
        }        
        result.to_string()
    }

    fn part_02(&self) -> String {
        let symbols: Vec<Point<usize, &Space>> = self.engine_schematics
            .iter_points()
            .filter(|p| {
                match p.value {
                    Space::Symbol('*') => true,
                    _ => false
                }
        }).collect();
        let mut numbers: Vec<Point<usize, String>> = vec![];
        let mut start_coord: Option<C> = None;
        let mut number: String = String::new();
        let numbers_it = self.engine_schematics.iter_points();
        
        numbers_it.for_each(|p| {
            match p.value {
                Space::Digit(d) => {
                    if start_coord.is_none() {
                        start_coord = Some(p.coord);
                    }
                    number.push(*d);                    
                },
                _ => {
                    if number.len() > 0 {
                        let number_point: Point<usize, String> = Point::from_coord(start_coord.unwrap(), number.clone());
                        numbers.push(number_point);
                        number = String::new();
                        start_coord = None;
                    }
                }
            }
        });
        let mut result: u32 = 0;
        for symbol in symbols {
            let neighbour_coords: Vec<_> = DIRECTIONS
                .iter()
                .filter_map(|direction| {
                self.engine_schematics.get_neighbour(&symbol.coord, direction)
            }).collect();
            let neighbour_numbers: Vec<&Point<usize, String>> = numbers
                .iter().filter(|n| {
                    neighbour_coords.iter().any(|c| {
                        if n.coord.y == c.y && n.coord.x <= c.x && n.coord.x + n.value.len() - 1 >= c.x {
                            true
                        } else {
                            false
                        }
                    })                    
                }).collect();
            if neighbour_numbers.len() == 2 {
                result += neighbour_numbers.iter().map(|p| p.value.parse::<u32>().unwrap()).product::<u32>();
            }            
        }        
        result.to_string()
    }
}


