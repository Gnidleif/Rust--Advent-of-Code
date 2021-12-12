use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {
    input: Vec<String>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 10, run_sample).await?;

        Ok(Day {
            input: content.lines().map(|line| line.to_string()).collect::<Vec<_>>(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut result = 0;
        let mut signs = self.input.clone();
        for line in signs.iter() {
            let chars = line.chars().map(|c| c.to_string()).collect::<Vec<_>>();
            // println!("{:?}", chars);
        }

        result
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
        format!("Day10 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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