use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use aoc_lib::Point;

pub struct Day {
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
            width: v[0].len(),
            input: v.iter().flatten().map(|d| *d).collect(),
        })
    }

    fn adjacent_indices(p: Point, w: usize, len: usize) -> Vec<usize> {
        let dpos: Vec<(i32, i32)> = vec![
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, 0),
        ];
        let mut result: Vec<usize> = vec![];
        for i in 0..dpos.len() {
            let x = p.x as i32 + dpos[i].0;
            let y = p.y as i32 + dpos[i].1;
            let pos = (y * w as i32) + x;

            println!("({}, {}) = {}", x, y, pos);
            if pos >= 0 && pos < len as i32 {
                result.push(pos as usize);
            }
        }

        result
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        for x in (0..self.input.len()).step_by(self.width) {
            println!("{:?}", &self.input[x..x+self.width]);
            for y in x..x+self.width {
                let adjacent = Day::adjacent_indices(Point {x: x, y: y}, self.width, self.input.len());
                println!("{:?}", adjacent);
            };
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