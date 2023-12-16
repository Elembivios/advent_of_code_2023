use crate::utils::point::{Grid, Coord, Direction};
use std::iter::once;

pub struct ParabolicReflectorDish {
    grid: Grid<char>,
    // pillars: Vec<C>
}

impl crate::Advent for ParabolicReflectorDish {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let map = data.lines().map(|l| {
            l.chars().collect()
        }).collect();
        let grid = Grid::new(map);
        Self { grid }
    }
    
    fn part_01(&self) -> String {
        let mut grid = self.grid.clone();
        grid.tilt(Direction::N);
        grid.calculate_load_north().to_string()
    }

    fn part_02(&self) -> String {
        let mut grid = self.grid.clone();
        let mut patterns: Vec<Vec<C>> = vec![];
        let mut rotation = 0;
        let num_rotations = 1_000_000_000;
        while rotation < num_rotations {
            if rotation % 1_000_000 == 0{
                println!("Rotation: {}", rotation);
            }                   
            for dir in [Direction::N, Direction::W, Direction::S, Direction::E].into_iter() {                
                grid.tilt(dir);                
            }            
            let rocks: Vec<C> = grid.iter_points().filter_map(|p| {
                if *p.value == 'O' {
                    Some(p.coord)
                } else {
                    None
                }
            }).collect();
            let prev_pattern = patterns.iter().position(|p| *p == rocks);
            match prev_pattern {
                Some(prev_index) => {
                    let occurance_span = rotation - prev_index;
                    let remaining_rotations = num_rotations - rotation;
                    let remainder = remaining_rotations % occurance_span;
                    rotation = num_rotations - remainder;
                    patterns.clear();
                },
                None => patterns.push(rocks)
            }
            rotation += 1;
        }
        
        grid.calculate_load_north().to_string()
    }
}

type C = Coord<usize>;


impl Grid<char> {
    fn tilt(&mut self, direction: Direction) {
        let start_coord = match direction {
            Direction::E => Coord::new(0, 0),
            Direction::W => Coord::new(self.width - 1, 0),
            Direction::N => Coord::new(0, self.height - 1),
            Direction::S => Coord::new(0, 0),
            _ => unimplemented!()
        };
        let other_dir = match direction {
            Direction::N | Direction::S => Direction::E,
            Direction::E | Direction::W => Direction::S,
            _ => unimplemented!()
        };

        let mut swaps: Vec<(C, C)> = vec![];   
        let rorc_it = self.direction_iter(other_dir, start_coord); 
        for first_in_row_or_col in once(start_coord).chain(rorc_it) {
            let mut row_or_col_it = once(first_in_row_or_col).chain(self.direction_iter(direction, first_in_row_or_col)).peekable();
            let mut rocks: Vec<C> = vec![];
            while let Some(c) = row_or_col_it.next() {
                let v = self.get_val(&c);
                if *v == '#' {
                    let dir_it = self.direction_iter(direction.rotate(180), c);                    
                    rocks.drain(..).zip(dir_it).for_each(|e| {
                        swaps.push(e)
                    });
                } else if *v == 'O' {
                    rocks.push(c);
                }
                
                if row_or_col_it.peek().is_none() {
                    let dir_it = self.direction_iter(direction.rotate(180), c);                    
                    rocks.drain(..).zip(once(c).chain(dir_it)).for_each(|e| {
                        swaps.push(e)
                    });
                } 
            }            
        }
        swaps.iter().for_each(|(from, _)| {
            *self.get_val_mut(from) = '.';
        });
        swaps.iter().for_each(|(_, to)| {
            *self.get_val_mut(to) = 'O';
        });
    }

    fn calculate_load_north(&self) -> usize {
        self.iter_points().filter(|p| *p.value == 'O').map(|p| {
            self.height - p.coord.y
        }).sum()
    }
}