use std::{
    result::Result,
    error::Error,
};

enum Moves {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub struct Day {
    input: Vec<Moves>,
}

impl Day {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 2).await?;

        Ok(Day {
            input: content.lines()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .map(|x| {
                let n = x[1].parse().unwrap();
                match x[0] {
                    "forward" => Moves::Forward(n),
                    "up" => Moves::Up(n),
                    "down" => Moves::Down(n),
                    _ => unreachable!(),
                }
            }).collect(),
        })
    }

    #[allow(dead_code)]
    fn part1_old(&self) -> i32 {
        let mut point = (0, 0);
        for mv in self.input.iter() {
            match mv {
                Moves::Forward(n) => point.0 += n,
                Moves::Up(n) => point.1 -= n,
                Moves::Down(n) => point.1 += n,
            };
        }

        point.0 * point.1
    }

    #[allow(dead_code)]
    fn part2_old(&self) -> i32 {
        let mut aim = 0;
        let mut point = (0, 0);
        for mv in self.input.iter() {
            match mv {
                Moves::Forward(n) => {
                    point.0 += n;
                    point.1 += n * aim;
                },
                Moves::Up(n) => aim -= n,
                Moves::Down(n) => aim += n,
            };
        }

        point.0 * point.1
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let (x, y) = self.input.iter()
            .fold((0, 0), |(x, y), mv| match mv {
                Moves::Forward(n) => (x + n, y),
                Moves::Up(n) => (x, y - n),
                Moves::Down(n) => (x, y + n),
            });
        
        x * y
    }

    fn part2(&self) -> i32 {
        let (x, y, _) = self.input.iter()
            .fold((0, 0, 0), |(x, y, a), mv| match mv {
                Moves::Forward(n) => (x + n, y + a * n, a),
                Moves::Up(n) => (x, y, a - n),
                Moves::Down(n) => (x, y, a + n),
            });

        x * y
    }
    
    fn fmt_result(&self) -> String {
        format!("Day2 (2021): ({}, {})", self.part1(), self.part2())
    }
}

#[cfg(test)]
mod day2_testing {
    use aoc_lib::{Day, aw};

    fn new() -> super::Day {
        aw!(super::Day::new()).unwrap()
    }

    #[test]
    fn p1_old() {
        assert_eq!(1635930, new().part1_old());
    }

    #[test]
    fn p2_old() {
        assert_eq!(1781819478, new().part2_old());
    }

    #[test]
    fn p1() {
        assert_eq!(1635930, new().part1());
    }

    #[test]
    fn p2() {
        assert_eq!(1781819478, new().part2());
    }
}