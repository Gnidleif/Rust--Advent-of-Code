use std::{
    result::Result,
    error::Error,
};

struct Dimension(i32, i32, i32);

pub struct Day {
    input: Vec<Dimension>,
}

impl Day {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 2).await?;

        Ok(Day {
            input: content.lines()
                .map(|x| x.split("x").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                .map(|x| Dimension(x[0], x[1], x[2]))
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let sum = self.input.iter()
            .map(|Dimension(l, w, h)| vec![l * w, w * h, h * l])
            .fold(0, |acc, areas| {
                let smallest: i32 = match areas.iter().min() {
                    Some(n) => *n,
                    None => unreachable!(),
                };
                let total: i32 = areas.iter().sum();

                acc + (2 * total) + smallest
            });

        sum
    }

    fn part2(&self) -> i32 {

        0
    }

    fn fmt_result(&self) -> String {
        format!("Day2 (2015): ({}, {})", self.part1(), self.part2())
    }
}