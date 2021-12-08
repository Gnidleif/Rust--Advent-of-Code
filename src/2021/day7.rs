use std::{
    result::Result,
    error::Error,
    time::Instant,
    ops::Sub,
};
use aoc_lib::algorithms::binary_search;

pub struct Day {
    input: Vec<i32>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 7, run_sample).await?;

        let mut sorted: Vec<i32> = content.split(",").map(|x| x.parse().unwrap()).collect();
        sorted.sort();

        Ok(Day {
            input: sorted,
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let calc_score = |pos: &[i32], target: i32| pos.iter()
            .map(|p| p.sub(target).abs()).sum();

        binary_search(&self.input[..], calc_score) as usize
    }

    fn part2(&self) -> usize {
        let calc_score = |pos: &[i32], target: i32| pos.iter()
            .map(|p| { let dist = (p - target).abs(); 
                dist * (dist + 1) / 2 }).sum();

        binary_search(&self.input[..], calc_score) as usize
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day7 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(0, day.part1());
        assert_eq!(0, day.part2());
    }
}