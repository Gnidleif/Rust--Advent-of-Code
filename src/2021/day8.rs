use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::{
        HashSet,
        HashMap,
    },
};
use itertools::Itertools;

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

    fn generate_key(left: &[String]) -> HashMap<String, usize> {
        let base: HashMap<String, usize> = left.iter()
            .filter(|s| match s.len() { 5|6 => false, _ => true })
            .map(|s| 
                (s.to_string(), match s.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => unreachable!(),
                })
            ).collect::<HashMap<String, usize>>();

        let key4 = HashSet::from_iter((*base.iter().find(|(_, v)| **v == 4).unwrap().0).chars());
        let key7 = HashSet::from_iter((*base.iter().find(|(_, v)| **v == 7).unwrap().0).chars());

        let mut key = left.iter()
            .filter(|s| match s.len() { 5|6 => true, _ => false })
            .map(|s| {
                (s.to_string(), match s.len() {
                    5 => if Day::xcount(&s, &key4) == 2 {
                        2
                    } else if Day::xcount(&s, &key7) == 3 {
                        3
                    } else {
                        5
                    },
                    6 => if Day::xcount(&s, &key4) == 4 {
                        9
                    } else if Day::xcount(&s, &key7) == 3 {
                        0
                    } else {
                        6
                    },
                    _ => unreachable!(),
                })
            }).collect::<HashMap<String, usize>>();
        key.extend(base);

        key
    }

    fn xcount(s: &str, b: &HashSet<char>) -> usize {
        let a: HashSet<char> = HashSet::from_iter(s.chars());
        a.intersection(&b).count()
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
            let both = &line.split("|")
                .map(|side| side.split_whitespace()
                    .map(|s| s.chars().sorted().collect::<String>()).collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>();

            let (left, right) = (&both[0..10], &both[10..14]);
            let key = Day::generate_key(left);

            sum += right.iter()
                .map(|word| key[word].to_string())
                .collect::<String>().parse::<usize>().unwrap();
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