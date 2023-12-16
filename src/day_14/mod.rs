use crate::utils::{point::{Grid, Point, Coord, Direction, Axis, TOUCHING_DIRECTIONS}, wait_user_input};
use std::{iter::once, collections::HashSet};

pub struct ParabolicReflectorDish {
    grid: Grid<char>,
    // rocks: Vec<C>,
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
        // let rocks: Vec<C> = grid.iter_points().filter(|p| *p.value == 'O').map(|p| p.coord).collect();
        // let pillars = grid.iter_points().filter(|p| *p.value == '#').map(|p| p.coord).collect();
        // grid.iter_points_mut().filter(|p| *p.value == 'O').for_each(|p| *p.value = '.');
        Self { grid }
    }
    
    fn part_01(&self) -> String {
        // let mut rocks = self.rocks.clone();
        let mut grid = self.grid.clone();
        grid.tilt(Direction::N);
        grid.calculate_load_north().to_string()

        // 1.to_string()
    }

    fn part_02(&self) -> String {
        // let mut grid = self.grid.clone();
        // let mut prev_rotation: [Option<Grid<char>>; 4] = [None, None, None, None];
        // for rotation in 0..1_000_000_000 {
        //     if rotation % 1_000_000 == 0{
        //         println!("Rotation: {}", rotation);
        //     }                   
        //     for (i, dir) in [Direction::N, Direction::W, Direction::S, Direction::E].into_iter().enumerate() {
                
        //         grid.tilt(dir);
        //         match &prev_rotation[i] {
        //             Some(prev_grid) => {
        //                 if prev_grid.map == grid.map {
        //                     break;
        //                 } else {
        //                     let prev: HashSet<_> = prev_grid.iter_points().filter_map(|p| {
        //                         if *p.value == 'O' {
        //                             Some(p.coord)
        //                         } else {
        //                             None
        //                         }                                
        //                     }).collect();
        //                     let current: HashSet<_> = grid.iter_points().filter_map(|p| {
        //                         if *p.value == 'O' {
        //                             Some(p.coord)
        //                         } else {
        //                             None
        //                         }                                
        //                     }).collect();
        //                     println!("Diff: {:?}", prev.symmetric_difference(&current));
        //                     wait_user_input();
        //                 }
        //             },
        //             None => prev_rotation[i] = Some(grid.clone())
        //         }                
        //     }            
            
        // }
        
        // grid.calculate_load_north().to_string()
        2.to_string()
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