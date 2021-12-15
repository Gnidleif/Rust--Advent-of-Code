use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {
    input: Vec<usize>
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 6, run_sample).await?;
        Ok(Day {
            input: content.split(",").map(|c| c.parse().unwrap()).fold(vec![0; 9], |mut acc, v: usize| {
                acc[v] += 1;
                acc
            }),
        })
    }

    fn iterate_days(&self, num_days: usize) -> usize {
        let mut days = self.input.clone();
        for _ in 0..num_days {
            days.rotate_left(1);
            days[6] += days[8];
        }

        days.iter().sum()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        self.iterate_days(80)
    }

    fn part2(&self) -> usize {
        self.iterate_days(256)
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day6 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(349549, day.part1());
        assert_eq!(1589590444365, day.part2());
    }
}