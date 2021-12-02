#![allow(dead_code)]
use std::{
    result::Result,
    error::Error,
};

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
    input: Vec<(String, i64)>,
}

impl Day2 {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_input::create_input(2021, 2).await?;

        Ok(Day2 {
            input: content.lines()
                    .map(|x| x.split_whitespace().collect::<Vec<&str>>())
                    .map(|x| (x[0].to_string(), x[1].parse().unwrap()))
                    .collect(),
        })
    }
}

impl Day for Day2 {
    fn part1(&self) -> i64 {
        let mut pos: (i64, i64) = (0, 0);
        for (dir, len) in self.input.iter() {
            match &dir[..] {
                "forward" => pos.0 += len,
                "up" => pos.1 -= len,
                "down" => pos.1 += len,
                _ => continue,
            }
        }

        pos.0 * pos.1
    }

    fn part2(&self) -> i64 {
        let mut aim: i64 = 0;
        let mut pos: (i64, i64) = (0, 0);
        for (dir, len) in self.input.iter() {
            match &dir[..] {
                "forward" => {
                    pos.0 += len;
                    pos.1 += len * aim;
                },
                "up" => aim -= len,
                "down" => aim += len,
                _ => continue,
            }
        }

        pos.0 * pos.1
    }
}

// #endregion