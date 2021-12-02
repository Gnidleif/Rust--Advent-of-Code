#![allow(dead_code)]
use std::{
    result::Result,
    error::Error,
};

// macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
// }

pub trait Day {
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

// #region Day 1

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

    pub fn part1_old(&self) -> i64 {
        let mut sum = 0;
        for i in 1..self.input.len() {
            if self.input[i] > self.input[i - 1] {
                sum += 1;
            }
        }

        sum
    }
    
    pub fn part2_old(&self) -> i64 {
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

impl Day for Day1 {
    fn part1(&self) -> i64 {
        self.input
            .iter()
            .zip(self.input.iter().skip(1))
            .filter(|(a, b)| b > a)
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self) -> i64 {
        self.input
            .windows(4)
            .filter(|w| w[3] > w[0])
            .count()
            .try_into()
            .unwrap()
    }
}

// #endregion

// #region Day2

pub struct Day2 {

}

impl Day2 {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_input::create_input(2021, 2).await?;
        // let commands: Vec<Vec<String>> = content
        //     .lines()
        //     .map(|x| x.to_string().split_whitespace().map(|x| x.trim().to_string()).collect())
        //     .collect();
        let commands: Vec<Vec<String>> = content
            .lines()
            .map(|x| x.split_whitespace().map(|x| x.trim().to_string()).collect())
            .collect();

        for line in commands.iter() {
            println!("{:?}", line);
        }

        Ok(Day2 {

        })
    }
}

impl Day for Day2 {
    fn part1(&self) -> i64 {
        0
    }

    fn part2(&self) -> i64 {
        0
    }
}

// #endregion