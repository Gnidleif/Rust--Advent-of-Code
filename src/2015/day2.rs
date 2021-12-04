use std::{
    result::Result,
    error::Error,
    time::Instant,
};

struct Dimension(i32, i32, i32);

pub struct Day {
    input: Vec<Dimension>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 2).await?;

        Ok(Day {
            input: content.lines()
                .map(|x| x.split("x").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                .map(|x| Dimension(x[0], x[1], x[2]))
                .collect(),
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        self.input.iter()
        .map(|Dimension(l, w, h)| vec![l * w, w * h, h * l])
        .fold(0, |acc, areas| {
            let smallest: i32 = *areas.iter().min().unwrap();
            let total: i32 = areas.iter().sum();

            acc + (2 * total) + smallest
        })
    }

    fn part2(&self) -> i32 {
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
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day2 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}

#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new()).unwrap();
        assert_eq!(1598415, day.part1());
        assert_eq!(3812909, day.part2());
    }
}