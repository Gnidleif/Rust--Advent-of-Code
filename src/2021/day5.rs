use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use itertools::{
    Itertools,
    EitherOrBoth::{
        Left, 
        Right, 
        Both,
    },
};

pub struct Day {
    width: usize,
    height: usize,
    input: Vec<(Point, Point)>,
}

enum Range {
    Forward(std::ops::RangeInclusive<usize>),
    Backwards(std::iter::Rev<std::ops::RangeInclusive<usize>>),
}

impl Iterator for Range {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            Range::Forward(range) => range.next(),
            Range::Backwards(range) => range.next(),
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 5, run_sample).await?;

        // create an iterator for mapping the input to points
        let points = content.lines().map(|line| line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>())
            .map(|line| {
                let p1: Vec<usize> = line[0].split(",").map(|x| x.parse().unwrap()).collect();
                let p2: Vec<usize> = line[2].split(",").map(|x| x.parse().unwrap()).collect();
                (Point {x: p1[0], y: p1[1]}, Point {x: p2[0], y: p2[1]})
            });
        
        // find width and height of the board by finding highest x and y of iterated points
        let w = points.clone().map(|p| std::cmp::max(p.0.x, p.1.x)).max().unwrap();
        let h = points.clone().map(|p| std::cmp::max(p.1.y, p.1.y)).max().unwrap();

        Ok(Day {
            width: w + 1,
            height: h + 1,
            // fold each iterated point into a vector
            input: points.collect::<Vec<_>>(),
        })
    }

    fn create_range(start: usize, end: usize) -> Range {
        if start < end {
            Range::Forward(start..=end)
        }
        else {
            Range::Backwards((end..=start).rev())
        }
    }

    fn indices_from_points(&self, p1: &Point, p2: &Point) -> Vec<usize> {
        let xr = Day::create_range(p1.x, p2.x);
        let yr = Day::create_range(p1.y, p2.y);
        
        xr.zip_longest(yr).map(|i| match i {
                Both(x, y) => (x, y),
                Left(x) => (x, p1.y),
                Right(y) => (p1.x, y),
            }).map(|(x, y)| { 
                (y * self.width) + x}
            ).collect::<Vec<_>>()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let mut board = vec![0; self.width * self.height];
        for (p1, p2) in self.input.iter().filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y) {
            let indices = self.indices_from_points(p1, p2);
            for i in indices.iter() {
                board[*i] += 1;
            }
        }

        board.iter().filter(|v| **v >= 2).count() as i32
    }

    fn part2(&self) -> i32 {
        let mut board = vec![0; self.width * self.height];
        for (p1, p2) in self.input.iter() {
            let indices = self.indices_from_points(p1, p2);
            for i in indices.iter() {
                board[*i] += 1;
            }
        }

        board.iter().filter(|v| **v >= 2).count() as i32
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day5 (2021): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
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