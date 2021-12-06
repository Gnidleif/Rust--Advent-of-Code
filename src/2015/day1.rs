use std::{
    result::Result,
    error::Error,
    time::Instant,
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
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 1, run_sample).await?;

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
    fn part1(&self) -> usize {
        self.input.iter().sum::<i32>() as usize
    }

    fn part2(&self) -> usize {
        let (idx, _) = self.input.iter()
            .enumerate()
            .fold_while((0, 0), |acc, (i, x)| 
                if acc.1 < 0 {
                    Done(acc)
                }
                else {
                    Continue((i, acc.1 + x))
                }).into_inner();
        
        idx + 1
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day1 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(232, day.part1());
        assert_eq!(1783, day.part2());
    }
}