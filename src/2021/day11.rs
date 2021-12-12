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

    fn handle_flashes(octopi: &mut Vec<i32>, flashed: &mut HashSet<usize>, dim: &usize, delta: &Vec<(i32, i32)>) -> usize {
        let indices = octopi.iter()
            .enumerate()
            .filter(|(_, v)| **v > 9)
            .filter(|(i, _)| !flashed.contains(i))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        
        if indices.len() > 0 {
            for i in indices.into_iter() {
                flashed.insert(i);
                let x = (i % dim) as i32;
                let y = (i as i32 - x) / *dim as i32;

                let neighbors = delta.iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|(dx, _)| *dx >= 0 && *dx < *dim as i32)
                    .filter(|(_, dy)| *dy >= 0 && *dy < *dim as i32)
                    .map(|(dx, dy)| (dy * *dim as i32) + dx)
                    .filter(|j| *j >= 0 && *j < octopi.len() as i32)
                    .map(|j| j as usize)
                    .collect::<Vec<_>>();

                for j in neighbors.into_iter() {
                    octopi[j] += 1;
                }
            }
            return Day::handle_flashes(octopi, flashed, dim, delta);
        }

        for i in flashed.iter() {
            octopi[*i] = 0;
        }

        flashed.len()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut result = 0;
        let mut octopi = self.input.clone();
        
        for _ in 0..100 {
            for i in 0..octopi.len() {
                octopi[i] += 1;
            }
            result += Day::handle_flashes(&mut octopi, &mut HashSet::new(), &self.dimension, &self.delta);
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
            let _ = Day::handle_flashes(&mut octopi, &mut HashSet::new(), &self.dimension, &self.delta);
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
        assert_eq!(1735, day.part1());
        assert_eq!(400, day.part2());
    }
}