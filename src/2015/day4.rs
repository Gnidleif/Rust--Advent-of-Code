use std::{
    result::Result,
    error::Error,
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
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 4).await?;

        Ok(Day {
            input: content,
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let mut i: i32 = 0;
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

    fn part2(&self) -> i32 {
        let mut i: i32 = 0;
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
        format!("Day4 (2015): ({}, {})", self.part1(), self.part2())
    }
}


// #[cfg(test)]
// mod d415_testing {
//     use aoc_lib::{Day, aw};

//     fn new() -> super::Day {
//         aw!(super::Day::new()).unwrap()
//     }

//     #[test]
//     fn p1() {
//         assert_eq!(0, new().part1());
//     }

//     #[test]
//     fn p2() {
//         assert_eq!(0, new().part2());
//     }
// }