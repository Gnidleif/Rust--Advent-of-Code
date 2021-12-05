use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use onig::Regex;

pub struct Day {
    input: Vec<Command>,
}

#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(s: String) -> Self {
        let points: Vec<String> = s.split(",").map(|x| x.to_string()).collect();
        Point {
            x: points[0].parse().unwrap(),
            y: points[1].parse().unwrap(),
        }
    }
}

enum Command {
    TurnOn(Point, Point),
    Toggle(Point, Point),
    TurnOff(Point, Point),
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 6, run_sample).await?;

        let cmd_rgx = Regex::new(r"(on|off)").unwrap();
        let point_rgx = Regex::new(r"(\d+,\d+)").unwrap();

        Ok(Day {
            input: content.lines().map(|line| {
                let points: Vec<Point> = point_rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).map(|s| Point::from(s)).collect();
                let cmd: String = cmd_rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).collect();
                (cmd, points)
            })
            .map(|(s, points)| {
                let (p1, p2) = (points[0].clone(), points[1].clone());
                match &s[..] {
                    "on" => Command::TurnOn(p1, p2),
                    "off" => Command::TurnOff(p1, p2),
                    _ => Command::Toggle(p1, p2),
                }
            }).collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        self.input.iter()
            .fold(vec![vec![0; 1000]; 1000], |mut acc, cmd| {
                match cmd {
                    Command::TurnOn(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| acc[x][y] = 1)),
                    Command::TurnOff(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| acc[x][y] = 0)),
                    Command::Toggle(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| acc[x][y] = if acc[x][y] == 0 { 1 } else { 0 })),
                };
                acc
            }).iter().flatten().sum::<i32>()
    }

    fn part2(&self) -> i32 {
        self.input.iter()
            .fold(vec![vec![0; 1000]; 1000], |mut acc, cmd| {
                match cmd {
                    Command::TurnOn(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| acc[x][y] += 1)),
                    Command::TurnOff(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| if acc[x][y] > 0 { acc[x][y] -= 1 })),
                    Command::Toggle(p1, p2) => (p1.x..=p2.x).for_each(|x| 
                        (p1.y..=p2.y).for_each(|y| acc[x][y] += 2)),
                };
                acc
            }).iter().flatten().sum::<i32>()
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day6 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(543903, day.part1());
        assert_eq!(14687245, day.part2());
    }
}