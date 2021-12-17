use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::{
        HashMap,
    },
};

pub struct Day {
    input: Vec<char>,
    cmds: HashMap<String, char>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 14, run_sample).await?;

        let counts: HashMap<char, usize> = content.lines().next().unwrap().chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        println!("{:?}", counts);

        Ok(Day {
            input: content.lines()
                .next().unwrap().chars().collect(),
            cmds: content.lines().skip(2)
                .map(|line| {
                    let mut split = line.split(" -> ");
                    (split.next().unwrap().to_string(), split.next().unwrap().chars().next().unwrap())
                })
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        0
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
        format!("Day14 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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