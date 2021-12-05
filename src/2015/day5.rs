use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use onig::Regex;

pub struct Day {
    input: Vec<String>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 5, run_sample).await?;

        Ok(Day {
            input: content.lines().map(|x| x.to_string()).collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        vec![
            (Regex::new(r"ab|cd|pq|xy").unwrap(), false),
            (Regex::new(r"(\w)\1").unwrap(), true),
            (Regex::new(r"(.*[aeiou]){3,}").unwrap(), true),
        ].iter().fold(self.input.clone(), |mut acc, (rgx, expected)| {
            acc.retain(|line| rgx.find(line).is_some() == *expected);
            acc
        }).len() as i32
    }

    fn part2(&self) -> i32 {
        vec![
            Regex::new(r"(\w\w).*\1").unwrap(),
            Regex::new(r"(\w)\w\1").unwrap(),
        ].iter().fold(self.input.clone(), |mut acc, rgx| {
            acc.retain(|line| rgx.find(line).is_some());
            acc
        }).len() as i32
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day5 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(238, day.part1());
        assert_eq!(69, day.part2());
    }
}