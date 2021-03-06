use std::{
    result::Result,
    error::Error,
    time::Instant,
};

struct Dimension(usize, usize, usize);

pub struct Day {
    input: Vec<Dimension>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 2, run_sample).await?;

        Ok(Day {
            input: content.lines()
                .map(|x| x.split("x").map(|y| y.parse().unwrap()).collect::<Vec<_>>())
                .map(|x| Dimension(x[0], x[1], x[2]))
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        self.input.iter()
        .map(|Dimension(l, w, h)| vec![l * w, w * h, h * l])
        .fold(0, |acc, areas| {
            let smallest = *areas.iter().min().unwrap();
            let total: usize = areas.iter().sum();

            acc + (2 * total) + smallest
        })
    }

    fn part2(&self) -> usize {
        self.input.iter()
        .map(|Dimension(l, w, h)| {
            let mut smallest = vec![l, w, h];
            smallest.sort();
            (smallest[0] + smallest[1], l * w * h)
        })
        .fold(0, |acc, (dist, ribbon)| acc + 2 * dist + ribbon)
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day2 (2015): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(1598415, day.part1());
        assert_eq!(3812909, day.part2());
    }
}