pub struct MirageMaintenance {
    sequences: Vec<Vec<i64>>
}

impl crate::Advent for MirageMaintenance {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let sequences: Vec<Vec<i64>> = data
            .lines()
            .map(|l| {
                l
                    .split(" ")
                    .map(|d| d.parse().unwrap())
                    .collect()
            }).collect();

        Self { sequences }
    }

    fn part_01(&self) -> String {
        let mut histories: Vec<i64> = vec![];
        for sequence in &self.sequences {            
            let mut current_sequence = sequence.clone();            
            let mut last_values: Vec<i64> = vec![]; 
            while !current_sequence.iter().all(|n| *n == 0) {
                let diffs: Vec<i64> = current_sequence.windows(2).map(|w| {
                    w[1] - w[0]
                }).collect();
                last_values.push(*current_sequence.last().unwrap());
                current_sequence = diffs;                
            }
            let mut current_placeholder: i64 = 0;
            for last_value in last_values.into_iter().rev() {
                current_placeholder = current_placeholder + last_value;
            }
            histories.push(current_placeholder);
        }
        histories.iter().sum::<i64>().to_string()
    }

    fn part_02(&self) -> String {
        let mut histories: Vec<i64> = vec![];
        for sequence in &self.sequences {            
            let mut current_sequence = sequence.clone();            
            let mut first_values: Vec<i64> = vec![]; 
            while !current_sequence.iter().all(|n| *n == 0) {
                let diffs: Vec<i64> = current_sequence.windows(2).map(|w| {
                    w[1] - w[0]
                }).collect();
                first_values.push(*current_sequence.first().unwrap());
                current_sequence = diffs;                
            }
            let mut current_placeholder: i64 = 0;
            for first_value in first_values.into_iter().rev() {
                current_placeholder = first_value - current_placeholder;
            }
            histories.push(current_placeholder);
        }
        histories.iter().sum::<i64>().to_string()
    }
}