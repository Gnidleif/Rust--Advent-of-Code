use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};

pub struct Day {
    input: Vec<Direction>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Day {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 3).await?;

        Ok(Day {
            input: content.chars().map(|x| match x {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _ => unreachable!(),
            }).collect(),
        })
    }

    fn fold_helper(acc: &Vec<(i32, i32)>, dir: &Direction) -> (i32, i32) {
        let (x, y) = *acc.last().unwrap();
        match dir {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
        }
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        self.input.iter()
        .fold(vec![(0, 0)], |mut acc, dir| {
            acc.push(Day::fold_helper(&acc, dir));
            acc
        })
        .iter()
        .fold(HashSet::new(), |mut acc, point| {
            acc.insert(point);
            acc
        }).len() as i32
    }

    fn part2(&self) -> i32 {
        self.input.iter()
            .step_by(2)
            .fold(vec![(0, 0)], |mut acc, dir| {
                acc.push(Day::fold_helper(&acc, dir));
                acc
            })
            .iter().chain(self.input.iter()
                .skip(1)
                .step_by(2)
                .fold(vec![(0, 0)], |mut acc, dir| {
                    acc.push(Day::fold_helper(&acc, dir));
                    acc
                }).iter())
                .fold(HashSet::new(), |mut acc, point| {
                    acc.insert(point);
                    acc
                })
                .len() as i32
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day3 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new()).unwrap();
        assert_eq!(2081, day.part1());
        assert_eq!(2341, day.part2());
    }
}