use std::{
    result::Result,
    error::Error,
};

pub struct Day1 {
    input: Vec<u16>,
}

impl Day1 {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 1).await?;

        Ok(Day1 {
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

impl aoc_lib::Day for Day1 {
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
}

#[cfg(test)]
mod day1_testing {
    use super::Day1;
    use aoc_lib::{Day, aw};

    fn new() -> Day1 {
        aw!(Day1::new()).unwrap()
    }

    #[test]
    fn p1_old() {
        assert_eq!(1791, new().part1_old());
    }

    #[test]
    fn p2_old() {
        assert_eq!(1822, new().part2_old());
    }

    #[test]
    fn p1() {
        assert_eq!(1791, new().part1());
    }

    #[test]
    fn p2() {
        assert_eq!(1822, new().part2());
    }
}