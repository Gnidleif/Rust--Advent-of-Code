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
        let content = aoc_lib::create_input(2015, 8, run_sample).await?;

        Ok(Day {
            input: content.lines().map(|s| s.to_string()).collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut total_diff = 0;
        let rgx = Regex::new(r#"(\\)(x\d+|")?"#).unwrap();
        for line in self.input.iter(){
            let a_len = line.len();
            let slice = &line[1..a_len-1];
            let mut b_len = slice.len();
            if rgx.find(slice).is_some() {
                for c in rgx.captures(slice).unwrap().iter().last() {
                    let s = match c {
                        Some(s) => s,
                        None => "",
                    };
                    println!("{}, {}", s, s.len());
                }
            }
            println!("{}", slice);

            total_diff += a_len - b_len;
        }
        total_diff
    }

    fn part2(&self) -> usize {
        0
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day8 (2015): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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