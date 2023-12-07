use std::ops::Range;


pub struct IfYouGoveASeedAFertilizer {
    seeds: Vec<u64>,
    maps:  Vec<Vec<RangeMap>>
}

impl crate::Advent for IfYouGoveASeedAFertilizer {
    fn new(data: &str) -> Self
        where 
            Self: Sized {

        let parts: Vec<&str> = data.split("\n\n").collect();
        let mut parts_it = parts.iter();
        let (_, seed_values_s) = parts_it.next().unwrap().split_once(": ").unwrap();
        let seeds: Vec<u64> = seed_values_s.split(" ").map(|s| {s.parse().unwrap()}).collect();

        let mut maps: Vec<Vec<RangeMap>> = vec![];
        while let Some(part) = parts_it.next() {
            let mut part_it = part.lines();
            let (part_name, _) = part_it.next().unwrap().split_once(" ").unwrap();
            println!("Part name: {}", part_name);
            let mut ranges: Vec<RangeMap> = vec![];
            while let Some(range_str) = part_it.next() {
                let numbers: Vec<u64> = range_str.split(" ").map(|s| s.parse().unwrap()).collect();
                ranges.push(RangeMap { 
                    destination_start: numbers[0],
                    source_start: numbers[1],
                    range_length: numbers[2]
                });
            }
            maps.push(ranges);
        }
        Self {
            seeds,
            maps
        }
    }

    fn part_01(&self) -> String {
        let mut location_numbers: Vec<u64> = vec![];
        for seed in &self.seeds {
            let mut number = *seed;
            for ranges in &self.maps {
                number = transform_number(number, ranges);
            }
            location_numbers.push(number);
        }
        location_numbers.iter().min().unwrap().to_string()
    }

    fn part_02(&self) -> String {                
        let seed_ranges: Vec<Range<u64>> = self.seeds.chunks(2).map(|sr| {
            Range {start: sr[0], end: sr[0] + sr[1]}
        }).collect();
        let mut min_location: u64 = u64::MAX;
        for seed_range in seed_ranges {
            for seed in seed_range {
                let mut number = seed;
                for ranges in &self.maps {
                    number = transform_number(number, ranges);
                }
                if number < min_location {
                    min_location = number;
                }
            }
        }
        min_location.to_string()
    }
}

fn transform_number(n: u64, ranges: &Vec<RangeMap>) -> u64 {
    let transformed = ranges.iter().find_map(|r| {
        r.map_number(n)
    });
    if let Some(new_n) = transformed {
        new_n
    } else {
        n
    }

}

struct RangeMap {
    destination_start: u64,
    source_start: u64,
    range_length: u64
}

impl RangeMap {
    fn map_number(&self, n: u64) -> Option<u64> {
        if (self.source_start..self.source_start+self.range_length).contains(&n) {
            Some(n + self.destination_start - self.source_start)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_number() {
        let rm = RangeMap{destination_start:52, source_start: 50, range_length: 48};
        let res= rm.map_number(53).unwrap();
        assert_eq!(res, 55);
    }
}