use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};
use regex::Regex;
use aoc_lib::Point;

pub struct Day {
    width: usize,
    height: usize,
    folds: Vec<Fold>,
    input: HashSet<Point>,
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 13, run_sample).await?;
        let fold_rgx = Regex::new(r"^[\w\s]+(?P<fold>[xy])=(?P<at>\d+)$").unwrap();
        let point_rgx = Regex::new(r"^(?P<x>\d+),(?P<y>\d+)").unwrap();

        let points = content.lines()
            .filter_map(|line|
                match point_rgx.captures(line) {
                    Some(cap) => Some(Point {
                        x: cap["x"].parse::<usize>().unwrap(),
                        y: cap["y"].parse::<usize>().unwrap(),
                    }),
                    None => None,
            }).collect::<HashSet<_>>();

        let w: usize = points.iter().map(|p| p.x).max().unwrap() + 1;
        let h: usize = points.iter().map(|p| p.y).max().unwrap() + 1;

        Ok(Day {
            width: w,
            height: h,
            folds: content.lines()
                .filter_map(|line| 
                    match fold_rgx.captures(line) {
                        Some(cap) => match &cap["fold"] {
                            "x" => Some(Fold::X(cap["at"].parse::<usize>().unwrap())),
                            "y" => Some(Fold::Y(cap["at"].parse::<usize>().unwrap())),
                            _ => unreachable!(),
                        }
                        None => None,
                    }).collect::<Vec<_>>(),
            input: points,
        })
    }

    fn fold_y(n: usize, h: &mut usize, set: HashSet<Point>) -> HashSet<Point> {
        let result = set.iter().filter_map(|p| match p.y {
            y if y > n => Some(Point {
                    x: p.x,
                    y: (*h - 1) - y
                }),
            y if y < n => Some(*p),
            _ => None,
        }).collect::<HashSet<_>>();
        *h -= n + 1;

        result
    }

    fn fold_x(n: usize, w: &mut usize, set: HashSet<Point>) -> HashSet<Point> {
        let result = set.iter().filter_map(|p| match p.x {
            x if x > n => Some(Point {
                x: (*w - 1) - x,
                y: p.y,
            }),
            x if x < n => Some(*p),
            _ => None,
        }).collect::<HashSet<_>>();
        *w -= n + 1;

        result
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut grid = self.input.clone();
        let (mut w, mut h) = (self.width, self.height);

        grid = match self.folds[0] {
            Fold::Y(n) => Day::fold_y(n, &mut h, grid),
            Fold::X(n) => Day::fold_x(n, &mut w, grid),
        };
        
        grid.len()
    }

    fn part2(&self) -> usize {
        let mut grid = self.input.clone();
        let (mut w, mut h) = (self.width, self.height);

        for fold in self.folds.iter() {
            grid = match fold {
                Fold::Y(n) => Day::fold_y(*n, &mut h, grid),
                Fold::X(n) => Day::fold_x(*n, &mut w, grid),
            };
        }

        let mut text = String::new();
        for y in 0..h {
            text.push_str("■");
            for x in 0..w {
                text.push_str(if grid.contains(&Point{x: x, y: y}) {
                    " "
                } else {
                    "■"
                });
            }
            text.push_str("■\n");
        }
        println!("{}", text);

        grid.len()
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day13 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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