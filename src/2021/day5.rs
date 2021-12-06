use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashMap,
};
use aoc_lib::Point;

pub struct Day {
    width: usize,
    input: Vec<(Point, Point)>,
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

        Ok(Day {
            width: w + 1,
            // fold each iterated point into a vector
            input: points.collect::<Vec<_>>(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        self.input.iter().filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
            .fold(HashMap::new(), |mut map: HashMap<usize, usize>, (p1, p2)| {
                for i in aoc_lib::line_between_points(p1, p2, &self.width) {
                    map.insert(i, match map.get(&i) {
                        Some(n) => n + 1,
                        None => 1,
                    });
                }
                map
            }).iter().map(|(_, v)| v).filter(|v| **v > 1).count()
    }

    fn part2(&self) -> usize {
        self.input.iter()
            .fold(HashMap::new(), |mut map: HashMap<usize, usize>, (p1, p2)| {
                for i in aoc_lib::line_between_points(p1, p2, &self.width) {
                    map.insert(i, match map.get(&i) {
                        Some(n) => n + 1,
                        None => 1,
                    });
                }
                map
            }).iter().filter(|(_, v)| **v > 1).count()
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day5 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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