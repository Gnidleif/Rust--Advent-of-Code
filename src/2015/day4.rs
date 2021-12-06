use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use md5::{
    Md5,
    Digest,
};

pub struct Day {
    input: String,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 4, run_sample).await?;

        Ok(Day {
            input: content,
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut i = 0;
        'outer: loop {
            let s = format!("{}{}", self.input, i);
            let hash = Md5::digest(s.as_bytes());
            for j in 0..=1 {
                if hash[j] != 0 {
                    i += 1;
                    continue 'outer
                };
            };
            if hash[2] < 16 {
                return i;
            }
            i += 1;
        }
    }

    fn part2(&self) -> usize {
        let mut i = 0;
        'outer: loop {
            let s = format!("{}{}", self.input, i);
            let hash = Md5::digest(s.as_bytes());
            for j in 0..=1 {
                if hash[j] != 0 {
                    i += 1;
                    continue 'outer
                };
            };
            if hash[2] == 0 {
                return i;
            }
            i += 1;
        }
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day4 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(254575, day.part1());
        assert_eq!(1038736, day.part2());
    }
}