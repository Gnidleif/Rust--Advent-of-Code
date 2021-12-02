use std::{
    result::Result,
    error::Error,
};

pub struct Day {

}

impl Day {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 3).await?;

        Ok(Day {

        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        0
    }

    fn part2(&self) -> i32 {
        0
    }

    fn fmt_result(&self) -> String {
        format!("Day3 (2021): ({}, {})", self.part1(), self.part2())
    }
}

#[cfg(test)]
mod d315_testing {
    use aoc_lib::{Day, aw};

    fn new() -> super::Day {
        aw!(super::Day::new()).unwrap()
    }

    #[test]
    fn p1() {
        assert_eq!(0, new().part1());
    }

    #[test]
    fn p2() {
        assert_eq!(0, new().part2());
    }
}