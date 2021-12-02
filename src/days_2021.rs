#![allow(dead_code)]
use std::{
    result::Result,
    error::Error,
};

pub trait Day {
    fn part1(&self) -> i32;
    fn part2(&self) -> i32;
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

    pub fn part1_old(&self) -> i32 {
        let mut sum = 0;
        for i in 1..self.input.len() {
            if self.input[i] > self.input[i - 1] {
                sum += 1;
            }
        }

        sum
    }

    pub fn part2_old(&self) -> i32 {
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

// #endregion

// #region Day2

enum Day2Moves {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub struct Day2 {
    input: Vec<Day2Moves>,
}

impl Day2 {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_input::create_input(2021, 2).await?;

        Ok(Day2 {
            input: content.lines()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .map(|x| {
                let n = x[1].parse().unwrap();
                match x[0] {
                    "forward" => Day2Moves::Forward(n),
                    "up" => Day2Moves::Up(n),
                    "down" => Day2Moves::Down(n),
                    _ => unreachable!(),
                }
            }).collect(),
        })
    }
}

impl Day for Day2 {
    fn part1(&self) -> i32 {
        let mut pos: (i32, i32) = (0, 0);
        for mv in self.input.iter() {
            match mv {
                Day2Moves::Forward(n) => pos.0 += n,
                Day2Moves::Up(n) => pos.1 -= n,
                Day2Moves::Down(n) => pos.1 += n,
            }
        }

        pos.0 * pos.1
    }

    fn part2(&self) -> i32 {
        let mut aim = 0;
        let mut pos = (0, 0);
        for mv in self.input.iter() {
            match mv {
                Day2Moves::Forward(n) => {
                    pos.0 += n;
                    pos.1 += n * aim;
                },
                Day2Moves::Up(n) => aim -= n,
                Day2Moves::Down(n) => pos.2 += n,
            }
        }

        pos.0 * pos.1
    }
}

// #endregion