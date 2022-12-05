use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {

}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2022, 1, run_sample).await?;

        Ok(Day {

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
        let mut now = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now.elapsed().as_micros();
        now = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now.elapsed().as_micros();
        format!("Day2 (2022): ({}: {}μs, {}: {}μs) total: {}μs", p1, elapsed1, p2, elapsed2, elapsed1 + elapsed2)
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