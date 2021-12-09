use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use aoc_lib::Point;

pub struct Day {
    height: usize,
    width: usize,
    input: Vec<u32>
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 9, run_sample).await?;

        let v: Vec<Vec<u32>> = content.lines().map(|line| 
            line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();

        Ok(Day {
            height: v.len(),
            width: v[0].len(),
            input: v.iter().flatten().map(|d| *d).collect(),
        })
    }

    fn adjacent_indices(px: i32, py: i32, w: i32, l: i32) -> Vec<usize> {
        let dpos = vec![
            (0, -1),
            (-1, 0),
            (0, 1),
            (1, 0),
        ];
        let mut indices = Vec::new();
        for (dx, dy) in dpos.iter() {
            let y = py + dy;
            if y >= w || y < 0 {
                continue;
            }
            let x = px + dx;
            if x < 0 {
                continue;
            }
            let i = (y * w) + x;
            if i >= l {
                continue;
            }
            indices.push(i as usize);
        }

        indices
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        println!("{} x {} = {} | {}", self.width, self.height, self.width * self.height, self.input.len());
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                let indices = Day::adjacent_indices(x, y, self.width as i32, self.input.len() as i32);
            }
        }
        0
    }

    fn part2(&self) -> usize {
        0
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day9 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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