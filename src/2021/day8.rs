use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};
use itertools::{
    Itertools,
    FoldWhile::{
        Continue,
        Done,
    }
};

pub struct Day {
    input: Vec<String>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 8, run_sample).await?;

        Ok(Day {
            input: content.lines().map(|line| line.to_string()).collect::<Vec<_>>(),
        })
    }

    fn create_key(line: &[String]) -> Vec<HashSet<char>> {
        let base = line.iter()
            .filter(|word| match word.len() { 5|6 => false, _ => true })
            .map(|s| HashSet::from_iter(s.chars()))
            .fold_while(vec![HashSet::new(); 10], |mut acc, word| {
                let idx = match word.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => unreachable!(),
                };
                if acc[idx].len() == 0 {
                    acc[idx] = word;
                }

                if acc.iter().filter(|n| n.len() > 0).count() == 4 {
                    Done(acc)
                }
                else {
                    Continue(acc)
                }
            }).into_inner();

        line.iter()
            .filter(|word| match word.len() { 5|6 => true, _ => false })
            .map(|s| HashSet::from_iter(s.chars()))
            .fold_while(base.clone(), |mut acc, word| {
                let idx = match word.len() {
                    5 => if word.intersection(&base[4]).count() == 2 {
                        2
                    } else if word.intersection(&base[7]).count() == 3 {
                        3
                    } else {
                        5
                    },
                    6 => if word.intersection(&base[4]).count() == 4 {
                        9
                    } else if word.intersection(&base[7]).count() == 3 {
                        0
                    } else {
                        6
                    },
                    _ => unreachable!(),
                };
                if acc[idx].len() == 0 {
                    acc[idx] = word;
                }

                if acc.iter().filter(|n| n.len() > 0).count() == 10 {
                    Done(acc)
                }
                else {
                    Continue(acc)
                }
            }).into_inner()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut sum: usize = 0;
        for line in self.input.iter() {
            let right = &line.split("|").map(|s| s.to_string()).collect::<Vec<_>>()[1];
            let output = right.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

            sum += output.iter()
                .map(|word| match word.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => 10,
                }).filter(|n| *n < 10).count();
        }
        
        sum
    }

    fn part2(&self) -> usize {
        let mut sum: usize = 0;
        for line in self.input.iter() {    
            let both: Vec<String> = line.split("|").map(|word| word.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>()).flatten().collect::<Vec<_>>();
            let complete = Day::create_key(&both);

            let right = &line.split("|").map(|s| s.to_string()).collect::<Vec<_>>()[1];
            let output = right.split_whitespace().map(|s| HashSet::from_iter(s.chars())).collect::<Vec<_>>();

            let mut capture = String::new();
            for word in output.iter() {
                match complete.iter().enumerate().find(|(_, set)| word == *set) {
                    Some((i, _)) => capture.push_str(&i.to_string()[..]),
                    None => (),
                }
            }
            sum += capture.parse::<usize>().unwrap();
        }
        
        sum
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