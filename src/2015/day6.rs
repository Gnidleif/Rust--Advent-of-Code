use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use aoc_lib::Point;
use onig::Regex;

pub struct Day {
    width: usize,
    height: usize,
    commands: Vec<Command>,
    points: Vec<(Point, Point)>,
}

enum Command {
    TurnOn,
    Toggle,
    TurnOff,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 6, run_sample).await?;
        let cmd_rgx = Regex::new(r"(on|off)").unwrap();
        let point_rgx = Regex::new(r"(\d+,\d+)").unwrap();

        let cmds: Vec<Command> = content.lines().map(|line| {
                cmd_rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).collect::<String>()
            }).map(|cmd| match &cmd[..] {
                "on" => Command::TurnOn,
                "off" => Command::TurnOff,
                _ => Command::Toggle,
            }).collect();

        let pts: Vec<(Point, Point)> = content.lines().map(|line| {
            point_rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).map(|s| {
                let p: Vec<usize> = s.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                Point {
                    x: p[0],
                    y: p[1],
                }
            }).collect::<Vec<Point>>()
            }).map(|p| (p[0].clone(), p[1].clone())).collect();

        Ok(Day {
            width: 1000,
            height: 1000,
            commands: cmds,
            points: pts,
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut board = vec![0; self.width * self.height];
        for (c, (p1, p2)) in self.commands.iter().zip(self.points.clone()) {
            for i in (p1.x..=p2.x).map(|x| {
                (p1.y..=p2.y).map(move |y| (y * self.width) + x)
                }).flatten() {
                match c {
                    Command::TurnOn => board[i] = 1,
                    Command::TurnOff => board[i] = 0,
                    Command::Toggle => board[i] = if board[i] == 0 { 1 } else { 0 },
                }
            }
        }

        board.iter().sum()
    }

    fn part2(&self) -> usize {
        let mut board = vec![0; self.width * self.height];
        for (c, (p1, p2)) in self.commands.iter().zip(self.points.clone()) {
            for i in (p1.x..=p2.x).map(|x| {
                (p1.y..=p2.y).map(move |y| (y * self.width) + x)
                }).flatten() {
                match c {
                    Command::TurnOn => board[i] += 1,
                    Command::TurnOff => if board[i] > 0 { board[i] -= 1 },
                    Command::Toggle => board[i] += 2,
                }
            }
        }

        board.iter().sum()
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day6 (2015): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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