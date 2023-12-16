use std::fmt;
use crate::utils::point::{Grid, Point, Coord, Direction, TOUCHING_DIRECTIONS};
use std::collections::VecDeque;

type PP = Point<usize, Pipe>;
type C = Coord<usize>;
pub struct PipeMaze {
    grid: Grid<Space>
}

impl crate::Advent for PipeMaze {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let map: Vec<Vec<Space>> = data.lines().map(|l| {
            l.chars().map(|c| Space::try_from(c).unwrap()).collect()
        }).collect();

        let grid = Grid::new(map);
        Self { grid }
    }

    fn part_01(&self) -> String {
        let start = self.grid.iter_points().find(|p| {
            *p.value == Space::Start
        }).unwrap();
        let start_connected_pipes: [(PP, Direction); 2] = TOUCHING_DIRECTIONS.iter().filter_map(|d| {
            let nc = self.grid.get_neighbour(&start.coord, d);
            nc.and_then(|nc| {
                let p = self.grid.get_point(&nc);
                match p.value {
                    Space::Pipe(pipe) => {
                        let rev_d = &d.rotate(180);
                        if pipe.connects.contains(rev_d) {
                            return Some((Point::from_coord(nc, pipe.clone()), *rev_d));
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            })
        }).collect::<Vec<_>>().try_into().unwrap();

        let mut steps = 2; // Start + first pipe
        let mut  current = start_connected_pipes[0].clone();
        while let Some(next_pipe) = self.grid.next_pipe(&current.0, &current.1) {
            steps += 1;
            current = next_pipe;
        }
        (steps / 2).to_string()
    }

    fn part_02(&self) -> String {
        let start = self.grid.iter_points().find(|p| {
            *p.value == Space::Start
        }).unwrap();
        let start_connected_pipes: [(PP, Direction); 2] = TOUCHING_DIRECTIONS.iter().filter_map(|d| {
            let nc = self.grid.get_neighbour(&start.coord, d);
            nc.and_then(|nc| {
                let p = self.grid.get_point(&nc);
                match p.value {
                    Space::Pipe(pipe) => {
                        let rev_d = &d.rotate(180);
                        if pipe.connects.contains(rev_d) {
                            return Some((Point::from_coord(nc, pipe.clone()), *rev_d));
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            })
        }).collect::<Vec<_>>().try_into().unwrap();

        let mut path: Vec<(PP, Direction)> = vec![start_connected_pipes[0].clone()];
        let mut  current = start_connected_pipes[0].clone();
        
        while let Some(next_pipe) = self.grid.next_pipe(&current.0, &current.1) {
            path.push(next_pipe.clone());
            current = next_pipe;
        }

        let is_right_outer = self.grid.is_any_neighbour_outer(&path, &Side::Right);
        
        let side = if is_right_outer {
            Side::Left
        } else {
            Side::Right
        };
        let mut inside_coords: Vec<C> = vec![];

        path.iter().for_each(|(pipe, dir)| {
            let dirs = Grid::lr_neigbours(&pipe.value, dir, &side);
            let mut queue: VecDeque<_> = dirs.into_iter().filter_map(|d| {
                self.grid.get_neighbour(&pipe.coord, &d).and_then(|c| {
                    let space = self.grid.get_point(&c);
                    Some(space.coord)
                })
            }).collect();

            while let Some(current_coord) = queue.pop_front() {
                if !inside_coords.contains(&current_coord) && !path.iter().map(|s| s.0.coord).any(|c| c==current_coord) {
                    self.grid.neighbour_coords(&current_coord).into_iter().for_each(|c| {
                        queue.push_back(c);
                    });
                    inside_coords.push(current_coord)
                }

            }                        
        });
        inside_coords.retain(|c| *c!=start.coord);
        inside_coords.len().to_string()
    }
}

impl Grid<Space> {
    fn next_pipe(&self, current: &PP, came_from: &Direction) -> Option<(PP, Direction)> {
        current.value.connects.iter().filter(|d| *d != came_from).find_map(|d| {
            // println!("Cur: {}, Cf: {}, D: {}", current, came_from, d);
            let c = self.get_neighbour(&current.coord, d).unwrap();
            let s = self.get_val(&c);
            match s {
                Space::Pipe(p) => {
                    Some((Point::from_coord(c, p.clone()), d.rotate(180)))
                },
                _ => None
            }
        })
    }

    fn lr_neigbours(pipe: &Pipe, came_from: &Direction, side: &Side) -> Vec<Direction> { 
        match pipe.label {
            'F' | '7' | 'J' | 'L' => {
                match side {
                    Side::Left => {
                        if *came_from == pipe.connects[0] {
                            vec![
                                came_from.rotate(-90),
                                came_from.rotate(-135),
                                came_from.rotate(-180)
                            ]
                        } else {
                            vec![]
                        }
                    },
                    Side::Right => {
                        if *came_from == pipe.connects[0] {
                            vec![]                            
                        } else {
                            vec![
                                came_from.rotate(-90),
                                came_from.rotate(-135),
                                came_from.rotate(-180)
                            ]
                        }
                    }
                }
            },
            _ => {
                match side {
                    Side::Left => vec![came_from.rotate(90)],
                    Side::Right => vec![came_from.rotate(-90)]
                }
            }
        }
    }

    fn is_any_neighbour_outer(&self, path: &Vec<(Point<usize, Pipe>, Direction)>, side: &Side) -> bool {
        let outer_coord = path.iter().find_map(|(pipe, dir)| {
            let dirs = Grid::lr_neigbours(&pipe.value, dir, &side);
            let space = dirs.into_iter().find_map(|d| {
                self.get_neighbour(&pipe.coord, &d).and_then(|c| {
                    let space = self.get_point(&c);
                    match space.value {
                        Space::Ground => Some(space),
                        _ => None
                    }
                })
            });
            if let Some(space) = space {
                let c = space.coord;
                if c.x == 0 || c.x == self.width || c.y == 0 || c.y == self.height {
                    Some(c)
                } else {
                    None
                }
            } else {
                None
            }
        });
        outer_coord.is_some()
    }
}

#[derive(Debug)]
enum Side {
    Right,
    Left
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Pipe(Pipe),
    Start,
    Ground
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Ground => '.',
            Self::Start => 'S',
            Self::Pipe(p) => p.label
        };
        write!(f, "{}", c)
    }
}

impl TryFrom<char> for Space {
    type Error = PipeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => {
                let pipe = Pipe::new(value);
                pipe.map(|val| Self::Pipe(val))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pipe {
    label: char,
    connects: [Direction; 2]
}
type PipeError = String;

impl Pipe {
    fn new(label: char) -> Result<Self, PipeError> {
        use Direction::*;
        let connects = match label {
            '|' => [N, S],
            '-' => [W, E],
            'F' => [S, E],
            '7' => [W, S],
            'J' => [N, W],
            'L' => [E, N],                                    
            _ => Err(format!("Invalid label for pipe {}", label))?
        };
        Ok(Self { label, connects})
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}