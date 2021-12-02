use std::{
    result::Result,
    error::Error,
};

pub struct Day {
    input: Vec<i32>
}

impl Day {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 1).await?;

        Ok(Day {
            input: content.chars().map(|x| match x {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            }).collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let sum = self.input.iter()
            .fold(0, |acc, x| acc + x);

        sum
    }

    fn part2(&self) -> i32 {
        let (idx, _) = self.input.iter()
            .enumerate()
            .fold((0, 0), |acc, (i, x)| 
                if acc.1 < 0 {
                    acc
                }
                else {
                    (i, acc.1 + x)
                });

        (idx + 1) as i32
    }

    fn fmt_result(&self) -> String {
        format!("Day1 (2015): ({}, {})", self.part1(), self.part2())
    }
}