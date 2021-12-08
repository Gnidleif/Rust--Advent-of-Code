use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};

#[derive(Debug)]
struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

pub struct Day {
    input: Vec<Line>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 8, run_sample).await?;

        Ok(Day {
            input: content.lines().map(|line| {
                let split: Vec<String> = line.split("|").map(|x| x.to_string()).collect();
                Line {
                    input: split[0].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>(),
                    output: split[1].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>(),
                }
            }).collect(),
        })
    }

    // fn unique_chars(s: &str) -> String {
    //     let set: HashSet<char> = s.chars().collect();

    //     String::from_iter(set.iter())
    // }

    fn determine_numbers(line: &[String]) -> Vec<i32> {
        line.iter().fold(Vec::new(), |mut acc, s| {
            let num = match s.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => -1,
            };
            if num >= 0 {
                acc.push(num);
            }
            acc
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let outputs = self.input.iter().map(|line| line.output.to_owned()).flatten().collect::<Vec<_>>();
        
        Day::determine_numbers(&outputs).len()
    }

    fn part2(&self) -> usize {
        0
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day8 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(0, day.part1());
        assert_eq!(0, day.part2());
    }
}