pub struct WaitForIt {
    times: Vec<u32>,
    records: Vec<u32>
}

impl crate::Advent for WaitForIt {
    fn new(data: &str) -> Self
        where 
            Self: Sized {
        let mut lines_it = data.lines();
        let times: Vec<u32> = lines_it.next().unwrap().split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| {
            s.parse().unwrap()
        }).collect();
        let records: Vec<u32> = lines_it.next().unwrap().split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| {
            s.parse().unwrap()
        }).collect();
        println!("Times: {:?}", times);
        println!("Records: {:?}", records);
        Self {
            times, 
            records
        }
    }
    fn part_01(&self) -> String {
        let mut possible_records: Vec<u32> = vec![];
        for (time, record) in self.times.iter().zip(&self.records) {
            let mut num_ways_to_beat_record: u32 = 0;
            for t in 0..*time {
                let remaining_time = time-t;
                let speed = t;
                let distance = speed * remaining_time;
                // println!("Pressed at: {}, Distance: {}", t, distance);
                if distance > *record {
                    num_ways_to_beat_record += 1
                }                
            }
            possible_records.push(num_ways_to_beat_record);
        }
        // println!("Possible records: {:?}", possible_records);
        possible_records.into_iter().product::<u32>().to_string()
    }

    fn part_02(&self) -> String {        
        let time_s: String = self.times.iter().map(|d| d.to_string()).collect();
        let record_s: String = self.records.iter().map(|d| d.to_string()).collect();
        let time: u64 = time_s.parse().unwrap();
        let record: u64 = record_s.parse().unwrap();
        
        let mut num_ways_to_beat_record: u64 = 0;
        for t in 0..time {
            let remaining_time = time-t;
            let speed = t;
            let distance = speed * remaining_time;
            // println!("Pressed at: {}, Distance: {}", t, distance);
            if distance > record {
                num_ways_to_beat_record += 1
            }                
        }
        

        num_ways_to_beat_record.to_string()
    }
}