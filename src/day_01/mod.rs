pub struct Trebuchet {
    data: Vec<String>
}

impl crate::Advent for Trebuchet {
    fn new(data: &str) -> Self {        
        Self {
            data: data.lines().map(|l| l.to_owned()).collect()
        }
    }

    fn part_01(&self) -> String {

        let results: Vec<u32> = self.data.iter().map(|l| {
            let first = l.chars().into_iter().find(|c| c.is_ascii_digit()).unwrap();
            let last = l.chars().into_iter().rev().find(|c| c.is_ascii_digit()).unwrap();

            let s: String = [first, last].iter().collect();
            s.parse::<u32>().unwrap()
        }).collect();
        results.iter().sum::<u32>().to_string()
    }

    fn part_02(&self) -> String {
        let results: Vec<u32> = self.data.iter().map(|l| {
            let chars = l.chars().collect();
            let first = find_first(&chars);
            let last = find_last(&chars);
            first * 10 + last        
        }).collect();        
        results.iter().sum::<u32>().to_string()
    }
}


fn find_first(chars: &Vec<char>) -> u32 {
    const WRITTEN_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", 
        "six", "seven", "eight", "nine"
    ];
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap()
        } else {
            let word: Vec<&char> = chars[index..std::cmp::min(index + 5, chars.len())].iter().collect();
            let word: String = word.into_iter().collect();
            let res = WRITTEN_DIGITS.iter().find(|&w| {
                word.starts_with(*w)
            });
            if let Some(res) = res {                
                let i =  WRITTEN_DIGITS.iter().position(|&w| w == *res).unwrap();
                return i as u32 + 1;
            }
        }
        index += 1;
    }

    panic!("No first found")
}


fn find_last(chars: &Vec<char>) -> u32 {
    const WRITTEN_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", 
        "six", "seven", "eight", "nine"
    ];
    let mut index = chars.len() as i32 - 1;
    while index >= 0 {
        let c = chars[index as usize];
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap()
        } else {
            let word: Vec<&char> = chars[std::cmp::max(index - 4, 0) as usize..=index as usize].iter().collect();
            let word: String = word.into_iter().collect();
            let res = WRITTEN_DIGITS.iter().find(|&w| {
                word.ends_with(*w)
            });
            if let Some(res) = res {                
                let i =  WRITTEN_DIGITS.iter().position(|&w| w == *res).unwrap();
                return i as u32 + 1;
            }
        }
        index -= 1;
    }

    panic!("No last found")
}