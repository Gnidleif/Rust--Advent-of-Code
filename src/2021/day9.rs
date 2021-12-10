use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashMap,
};

#[derive(Debug)]
struct Neighborhood {
    indices: HashMap<(i32, i32), usize>,
}

impl Neighborhood {
    fn new(x: i32, y: i32, w: i32, l: i32) -> Self {
        let m: HashMap<(i32, i32), usize> = vec![
            (0, -1),
            (-1, 0),
            (0, 0),
            (0, 1),
            (1, 0),
        ].iter().map(|(dx, dy)| ((dx, dy), ((y + dy) * w) + (x + dx)))
            .filter(|(_, i)| *i >= 0 && *i < l)
            .map(|((dx, dy), i)| ((*dx, *dy), i as usize))
            .collect::<HashMap<_, _>>();

        Neighborhood {
            indices: m,
        }
    }
}

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

        let h = v.len();
        let w = v[0].len();
        
        Ok(Day {
            height: h,
            width: w,
            input: v.iter().flatten().map(|d| *d).collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let neighborhoods = (0..self.width as i32).map(|x| 
            (0..self.height as i32).map(|y| {
                Neighborhood::new(x, y, self.width as i32, self.input.len() as i32)
            }).collect::<Vec<_>>()).flatten();
        
        let mut result = 0;
        for nb in neighborhoods {
            let me = self.input[nb.indices[&(0, 0)]];
            match nb.indices.iter().find(|(_, i)| self.input[**i] < me) {
                Some(_) => continue,
                None => result += me + 1,
            };
        }

        result as usize
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