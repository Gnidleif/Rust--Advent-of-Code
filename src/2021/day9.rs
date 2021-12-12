use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashMap,
};

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

    fn generate_neighbors(x: i32, y: i32, w: i32, h: i32, l: i32) -> HashMap<(i32, i32), usize> {
        vec![
            (0, -1),
            (-1, 0),
            (0, 0),
            (0, 1),
            (1, 0),
        ].into_iter().map(|(kx, ky)| (kx, ky, x + kx, y + ky))
            .filter(|(_, _, dx, _)| *dx >= 0 && *dx < w)
            .filter(|(_, _, _, dy)| *dy >= 0 && *dy < h)
            .map(|(kx, ky, dx, dy)| ((kx, ky), (dy * w) + dx))
            .filter(|(_, i)| *i >= 0 && *i < l)
            .map(|(p, i)| (p, i as usize))
            .collect::<HashMap<_, _>>()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let maps = (0..self.height as i32).map(|y| 
            (0..self.width as i32).map(|x| 
                Day::generate_neighbors(x, y, self.width as i32, self.height as i32, self.input.len() as i32))
                .collect::<Vec<_>>()).flatten().collect::<Vec<_>>();

        let mut result = 0;
        for nb in maps.iter() {
            let me = self.input[nb[&(0, 0)]];
            match nb.iter().find(|(_, i)| self.input[**i] < me) {
                Some(_) => continue,
                None => result += me + 1
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