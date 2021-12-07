use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};
use itertools::{
    Itertools,
    FoldWhile::{Continue, Done},
};

pub struct Day {
    input: Vec<usize>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 7, run_sample).await?;

        Ok(Day {
            input: content.split(",")
                .map(|x| x.parse().unwrap())
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let cpy = self.input.clone();
        let w = self.input.len();
        let mut low = usize::MAX;
        let mut set: HashSet<usize> = HashSet::new();
        for x in 0..w {
            let v = cpy[x];
            if set.contains(&v) {
                continue;
            }
            set.insert(v);

            match (0..(w - 1)).map(|y| (y + (x + 1)) % w).fold_while(0, |mut s, i| {
                s += if v > cpy[i] { v - cpy[i] } else { cpy[i] - v };
                if s > low {
                    Done(s)
                } else {
                    Continue(s)
                }
            }) {
                Done(_) => continue,
                Continue(sum) => low = sum,
            };
        }
        low
    }

    fn part2(&self) -> usize {
        let max = self.input.iter().max().unwrap();
        let pts: Vec<usize> = (0..=*max).map(|i| self.input.iter().filter(|n| **n == i).count()).collect();
        let w = pts.len();
        let mut low = usize::MAX;

        for x in 0..pts.len() {
            match (0..(w - 1)).map(|y| (y + (x + 1)) % w).fold_while(0, |mut s, y| {
                let diff = if x > y { x - y } else { y - x};
                s += (0..diff).map(|i| i + 1).sum::<usize>() * pts[y];
                if s > low {
                    Done(s)
                } else {
                    Continue(s)
                }
            }) {
                Done(_) => continue,
                Continue(sum) => low = sum,
            };
        }

        low
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