use std::{
    result::Result,
    error::Error,
};
use itertools::{
    Itertools,
    FoldWhile::{Continue, Done},
};

pub struct Day {
    input: Vec<i32>
}

impl Day {
    #[allow(dead_code)]
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
        self.input.iter().sum::<i32>()
    }

    fn part2(&self) -> i32 {
        let (idx, _) = self.input.iter()
            .enumerate()
            .fold_while((0, 0), |acc, (i, x)| 
                if acc.1 < 0 {
                    Done(acc)
                }
                else {
                    Continue((i, acc.1 + x))
                }).into_inner();

        (idx + 1) as i32
    }

    fn fmt_result(&self) -> String {
        format!("Day1 (2015): ({}, {})", self.part1(), self.part2())
    }
}


#[cfg(test)]
mod d115_testing {
    use aoc_lib::{Day, aw};

    fn new() -> super::Day {
        aw!(super::Day::new()).unwrap()
    }

    #[test]
    fn p1() {
        assert_eq!(232, new().part1());
    }

    #[test]
    fn p2() {
        assert_eq!(1783, new().part2());
    }
}