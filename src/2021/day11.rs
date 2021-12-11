use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashSet,
};

pub struct Day {
    delta: Vec<(i32, i32)>,
    dimension: usize,
    input: Vec<i32>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 11, run_sample).await?;
        
        Ok(Day {
            delta: (-1..=1).map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flatten().filter(|(dx, dy)| *dx != 0 || *dy != 0)
                .collect::<Vec<_>>(),
            dimension: 10,
            input: content.lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>())
                .flatten().collect(),
        })
    }

    fn run_flashes(&self, octopi: &mut Vec<i32>) -> HashSet<usize> {
        let mut flashed: HashSet<usize> = HashSet::new();
        let mut indices = octopi.iter()
            .enumerate()
            .filter(|(_, v)| **v > 9)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        while indices.len() > 0 {
            for i in indices.into_iter() {
                flashed.insert(i);
                let x = (i % self.dimension) as i32;
                let y = (i as i32 - x) / self.dimension as i32;

                let neighbors = self.delta.iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|(dx, _)| *dx >= 0 && *dx < self.dimension as i32)
                    .filter(|(_, dy)|  *dy >= 0 && *dy < self.dimension as i32)
                    .map(|(dx, dy)| (dy * self.dimension as i32) + dx)
                    .filter(|j| *j >= 0 && *j < octopi.len() as i32)
                    .collect::<Vec<_>>();

                for j in neighbors.iter() {
                    octopi[*j as usize] += 1;
                }
            }

            indices = octopi.iter().enumerate()
                .filter(|(_, v)| **v > 9)
                .filter(|(i, _)| !flashed.contains(i))
                .map(|(i, _)| i).collect::<Vec<_>>();
        }

        flashed
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut result = 0;
        let mut octopi = self.input.clone();
        
        for _s in 0..100 {
            for i in 0..octopi.len() {
                octopi[i] += 1;
            }

            let flashed = self.run_flashes(&mut octopi);
            for i in flashed.into_iter() {
                result += 1;
                octopi[i] = 0;
            }
        }

        result
    }

    fn part2(&self) -> usize {
        let mut result = 0;
        let mut octopi = self.input.clone();

        while octopi.iter().sum::<i32>() > 0 {
            for i in 0..octopi.len() {
                octopi[i] += 1;
            }

            let flashed = self.run_flashes(&mut octopi);
            for i in flashed.into_iter() {
                octopi[i] = 0;
            }
            result += 1;
        }
        
        result
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day11 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
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