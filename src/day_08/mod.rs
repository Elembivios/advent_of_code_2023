use std::collections::HashMap;
pub struct HauntedWateland {
    directions: Vec<char>,
    nodes: Vec<(String, [String; 2])>
}

impl crate::Advent for HauntedWateland {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let mut lines_it = data.lines();
        let directions: Vec<char> = lines_it.next().unwrap().chars().collect(); 
        lines_it.next();
        let mut nodes: Vec<(String, [String; 2])> = vec![];
        while let Some(l) = lines_it.next() {
            let (node_name, rhs) = l.split_once(" = ").unwrap();
            let (left, right) = rhs
                .strip_prefix("(").unwrap()
                .strip_suffix(")").unwrap()
                .split_once(", ").unwrap();

            nodes.push((node_name.to_owned(), [left.to_owned(), right.to_owned()]))
        }
        
        Self {
            directions,
            nodes
        }
    }
    
    fn part_01(&self) -> String {
        let dir_iter = self.directions.iter().cycle();
        let mut current_node = self.nodes.iter().find(|n| n.0 == "AAA").unwrap();        
        let mut steps: u32 = 0;
        for d in dir_iter {
            if current_node.0 == "ZZZ" {
                break;
            }
            let next_node = match d {
                'L' => &current_node.1[0],
                'R' => &current_node.1[1],
                _ => panic!("Invalid direction {}", d)
            };
            current_node = self.nodes.iter().find(|n| n.0 == *next_node).unwrap();            
            steps += 1;
        }
        steps.to_string()
    }

    fn part_02(&self) -> String {
        let dir_iter = self.directions.iter().cycle();
        let nodes: HashMap<String, [String; 2]> = self.nodes.iter().cloned().collect();
        let mut current_nodes: Vec<(&String, &[String; 2])> = nodes.iter().filter(|n| n.0.ends_with('A')).collect();
        let mut end_occurances: Vec<HashMap<String, Vec<u64>>> = current_nodes.iter().map(|_| HashMap::new()).collect();
        let mut patterns: Vec<u64> = vec![];
        let mut steps: u64 = 0;

        for d in dir_iter {
            if end_occurances.iter().all(|h| h.values().next().unwrap_or(&vec![]).len() > 2) {
                end_occurances.iter().for_each(|h| {
                    // Should only be values of one key
                    let values = h.values().next().unwrap();                    
                    let diff = values[2] - values[1];
                    patterns.push(diff);                    
                });
                let min_mul = recurse(&mut patterns);
                return min_mul.to_string();                
            }
            current_nodes.iter().enumerate().filter(|(_i,n)| n.0.ends_with('Z')).for_each(|(i, n)| {
                (*end_occurances[i].entry(n.0.clone()).or_default()).push(steps);
            });
            let mut new_nodes = Vec::with_capacity(current_nodes.len());
            for current_node in current_nodes.into_iter() {                
                let next_node = match d {
                    'L' => &current_node.1[0],
                    'R' => &current_node.1[1],
                    _ => panic!("Invalid direction {}", d)
                };
                new_nodes.push((next_node, nodes.get(next_node).unwrap()));
            }
            current_nodes = new_nodes;
            steps += 1;
        }
        unreachable!("Unreachable!");
    }
}


fn gfc(a: u64, b: u64) -> u64{
    if b == 0 {
        a
    } else {
        gfc(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gfc(a, b)
}

fn recurse(ar: &mut Vec<u64>) -> u64 {
    if ar.len() > 1 {
        let first = ar.pop().unwrap();
        let second = ar.pop().unwrap();
        ar.push(lcm(first, second));
        return recurse(ar);
    } else {
        return ar[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]    
    fn test_multiple_of() {
        let patterns = [(16579, 232106), (20513, 225643), (12083, 241660), (22199, 244189), (14893, 238288), (13207, 237726)];
        let steps = [16579, 20513, 12083, 22199, 14893, 13207];


        println!("{}", lcm(16579, 20513));        
        let mut steps_v: Vec<_> = steps.iter().cloned().collect();
        println!("{:?}", recurse(&mut steps_v));
    }
}