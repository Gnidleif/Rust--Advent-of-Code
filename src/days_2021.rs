use std::{
    result::Result,
    error::Error,
};

/* #region Day1 */

pub struct Day1 {
    input: Vec<u16>,
}

impl Day1 {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_input::create_input(2021, 1).await?;

        Ok(Day1 {
            input: content.lines().map(|x| x.parse::<u16>().unwrap()).collect(),
        })
    }
}

impl aoc_input::Day for Day1 {
    fn part1(&self) -> i64 {
        let mut sum = 0;
        for i in 1..self.input.len() {
            if self.input[i] > self.input[i - 1] {
                sum += 1;
            }
        }

        sum
    }

    fn part2(&self) -> i64 {
        let mut sum = 0;
        let mut last = self.input[0..3].iter().sum::<u16>();
        for i in 1..self.input.len() - 2 {
            let current = self.input[i..i+3].iter().sum::<u16>();
            if current > last {
                sum += 1;
            }
            last = current;
        }

        sum
    }
}

/* #endregion */