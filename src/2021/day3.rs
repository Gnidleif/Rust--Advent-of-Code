use std::{
    result::Result,
    error::Error,
    time::Instant,
};

pub struct Day {
    height: usize,
    width: usize,
    input: Vec<Vec<i32>>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 3).await?;

        let h = content.lines().count();
        let w = content.lines().last().unwrap().len();

        Ok(Day {
            height: h,
            width: w,
            input: content.lines()
                .map(|x| x.chars().map(|y| match y {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                }).collect::<Vec<i32>>()).collect()
        })
    }

    fn solve_rating(lines: &Vec<Vec<i32>>, common: bool) -> i32 {
        let mut pos = 0;
        let mut values = lines.clone();
        while values.len() > 1 {
            let num_ones = values.iter().map(|x| x.iter().nth(pos).unwrap()).filter(|x| **x == 1).count();
            if num_ones >= (values.len() - num_ones) {
                values.retain(|x| *x.iter().nth(pos).unwrap() == if common { 1 } else { 0 });
            }
            else {
                values.retain(|x| *x.iter().nth(pos).unwrap() == if common { 0 } else { 1 });
            }
            pos += 1;
        }
        let s_val = values[0].iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        });
        
        i32::from_str_radix(&s_val, 2).unwrap()
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let result = self.input.iter()
            .fold(vec![0; self.width], |mut acc, line| {
                line.iter().enumerate().for_each(|(i, &b)| acc[i] += b);
                acc
            }).iter()
            .fold(vec![String::new(), String::new()], |mut s, acc| {
                if *acc > (self.height / 2) as i32 {
                    s[0].push_str("1");
                    s[1].push_str("0");
                } else {
                    s[0].push_str("0");
                    s[1].push_str("1");
                }
                s
            });

        i32::from_str_radix(&result[0], 2).unwrap() * i32::from_str_radix(&result[1], 2).unwrap()
    }

    fn part2(&self) -> i32 {
        Day::solve_rating(&self.input, true) * Day::solve_rating(&self.input, false)
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day3 (2021): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new()).unwrap();
        assert_eq!(3687446, day.part1());
        assert_eq!(4406844, day.part2());
    }
}