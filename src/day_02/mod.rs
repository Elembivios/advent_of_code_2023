// 2141  --- too low


pub struct CubeConundrum {
    games: Vec<Vec<[u32;3]>>
}

impl crate::Advent for CubeConundrum {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let games = data.lines().map(|l| {
            let (_, rhs) = l.split_once(": ").unwrap();
            rhs.split("; ").map(|draw| {
                let mut color: [u32; 3] = [0, 0, 0];  
                draw.split(", ").for_each(|cube_str| {
                    let (num, color_str) = cube_str.split_once(' ').unwrap();                                      
                    let num: u32 = num.parse().unwrap();
                    match color_str {
                        "red" => color[0] = num,
                        "green" => color[1] = num,
                        "blue" => color[2] = num,
                        _ => panic!("Invalid color string: {}", color_str)
                    };                    
                });
                color
            }).collect()
        }).collect();
        Self { games }
    }

    fn part_01(&self) -> String {
        let max = [12, 13, 14];
        let possible_games: Vec<usize> = self.games.iter().enumerate().filter_map(|(i, draws)| {
            if draws.iter().all(|color| {
                color.iter().zip(max).all(|(v, m)| {
                    *v <= m
                })
            }) {
                Some(i+1)
            } else {
                None
            }
        }).collect();
        possible_games.iter().sum::<usize>().to_string()
    }

    fn part_02(&self) -> String {
        let fewest_colors: Vec<u32> = self.games.iter().map(|draws| {
            let mut min = [0; 3];
            draws.iter().for_each(|color| {
                color.iter().enumerate().for_each(|(i, &v)| {
                    if v > min[i] {
                        min[i] = v;
                    }
                })
            });
            min[0] * min[1] * min[2]
        }).collect();
        fewest_colors.iter().sum::<u32>().to_string()
    }
}
