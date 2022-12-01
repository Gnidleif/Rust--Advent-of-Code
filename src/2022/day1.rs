use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {
    input: Vec<Vec<usize>>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2022, 1, run_sample).await?;

        Ok(Day {
            input: content
                .split("\r\n\r\n")
                .map(|line| line.lines().map(|x| x.parse::<usize>().unwrap()).collect())
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        self.input
            .iter()
            .map(|lines| lines.iter().sum::<usize>())
            .max()
            .unwrap()
    }

    fn part2(&self) -> usize {
        let mut sums = self.input
            .iter()
            .map(|lines| lines.iter().sum::<usize>())
            .collect::<Vec<usize>>();

        sums.sort_by(|a, b| b.cmp(a));

        sums[0..=2].iter().sum::<usize>()
    }

    fn fmt_result(&self) -> String {
        let mut now = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now.elapsed().as_micros();
        now = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now.elapsed().as_micros();
        format!("Day1 (2022): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(69795, day.part1());
        assert_eq!(208437, day.part2());
    }
}