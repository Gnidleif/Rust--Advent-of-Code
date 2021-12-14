use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use regex::Regex;
use aoc_lib::Point;

pub struct Day {
    width: usize,
    height: usize,
    folds: Vec<Fold>,
    input: Vec<bool>,
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
        let fold_rgx = Regex::new(r"^[\w\s]+(?P<fold>[xy])=(?P<amount>\d+)$").unwrap();
        let point_rgx = Regex::new(r"^(?P<x>\d+),(?P<y>\d+)").unwrap();

        let points = content.lines()
            .filter_map(|line|
                match point_rgx.captures(line) {
                    Some(cap) => Some(Point {
                        x: cap["x"].parse::<usize>().unwrap(),
                        y: cap["y"].parse::<usize>().unwrap(),
                    }),
                    None => None,
            }).collect::<Vec<_>>();

        let w: usize = points.iter().map(|p| p.x).max().unwrap() + 1;
        let h: usize = points.iter().map(|p| p.y).max().unwrap() + 1;

        Ok(Day {
            width: w,
            height: h,
            folds: content.lines()
                .filter_map(|line| 
                    match fold_rgx.captures(line) {
                        Some(cap) => match &cap["fold"] {
                            "x" => Some(Fold::X(cap["amount"].parse::<usize>().unwrap())),
                            "y" => Some(Fold::Y(cap["amount"].parse::<usize>().unwrap())),
                            _ => unreachable!(),
                        }
                        None => None,
                    }).collect::<Vec<_>>(),
            input: points.iter()
                .fold(vec![false; w * h], |mut acc, p| {
                    let i = (p.y * w) + p.x;
                    acc[i] = true;
                    acc
                }),
        })
    }

    fn fold_x(n: usize, w: &mut usize, h: usize, grid: Vec<bool>) -> Vec<bool> {
        let w2 = *w - (n + 1);

        Vec::new()
    }

    fn fold_y(n: usize, w: usize, h: &mut usize, grid: Vec<bool>) -> Vec<bool> {
        let h2 = *h - (n + 1);
        let v = grid.iter().enumerate().filter(|(_, v)| **v)
            .filter_map(|(i, _)| {
                let x = i % w;
                let y = (i - x) / w;
                match y {
                    y if y > n => {
                        let y = (*h - 1) - y;
                        let j = (y * w) + x;
                        Some(j)
                    },
                    y if y < n => Some(i),
                    _ => None,
                }
            })
            .fold(vec![false; h2 * w], |mut v, idx| {
                v[idx] = true;
                v
            });

        *h = h2;
        v
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut grid = self.input.clone();
        let (mut w, mut h) = (self.width, self.height);
        grid = match self.folds[0] {
            Fold::X(n) => Day::fold_x(n, &mut w, h, grid),
            Fold::Y(n) => Day::fold_y(n, w, &mut h, grid),
        };

        println!("({}, {})", w, h);
        for i in (0..grid.len()).step_by(self.width) {
            let present = &grid[i..i+self.width].iter().map(|b| if *b { "#" } else { " " }).collect::<Vec<_>>();
            println!("{:?}", present);
        }

        grid.iter().map(|v| if *v { 1 } else { 0 }).sum()
    }

    fn part2(&self) -> usize {
        let mut grid = self.input.clone();
        let (mut w, mut h) = (self.width, self.height);
        println!("Before:\n({}, {})", w, h);
        for i in (0..grid.len()).step_by(w) {
            let present = &grid[i..i+w].iter().map(|b| if *b { "#" } else { " " }).collect::<Vec<_>>();
            println!("{:?}", present);
        }
        for fold in self.folds.iter() {
            grid = match fold {
                Fold::X(n) => Day::fold_x(*n, &mut w, h, grid),
                Fold::Y(n) => Day::fold_y(*n, w, &mut h, grid),
            };

            println!("({}, {})", w, h);
            for i in (0..grid.len()).step_by(w) {
                let present = &grid[i..i+w].iter().map(|b| if *b { "#" } else { " " }).collect::<Vec<_>>();
                println!("{:?}", present);
            }
        }
        0
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