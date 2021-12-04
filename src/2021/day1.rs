use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {
    input: Vec<u16>,
}

impl Day {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 1).await?;

        Ok(Day {
            input: content.lines().map(|x| x.parse::<u16>().unwrap()).collect(),
        })
    }

    #[allow(dead_code)]
    fn part1_old(&self) -> i32 {
        let mut sum = 0;
        for i in 1..self.input.len() {
            if self.input[i] > self.input[i - 1] {
                sum += 1;
            }
        }

        sum
    }

    #[allow(dead_code)]
    fn part2_old(&self) -> i32 {
        let mut sum = 0;
        let mut last = self.input[0..=2].iter().sum::<u16>();
        for i in 1..self.input.len() - 2 {
            let current = self.input[i..=i+2].iter().sum::<u16>();
            if current > last {
                sum += 1;
            }
            last = current;
        }

        sum
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        self.input
            .iter()
            .zip(self.input.iter().skip(1))
            .filter(|(a, b)| b > a)
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self) -> i32 {
        self.input
            .windows(4)
            .filter(|w| w[3] > w[0])
            .count()
            .try_into()
            .unwrap()
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day1 (2021): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new()).unwrap();
        assert_eq!(1791, day.part1_old());
        assert_eq!(1822, day.part2_old());
        assert_eq!(1791, day.part1());
        assert_eq!(1822, day.part2());
    }
}