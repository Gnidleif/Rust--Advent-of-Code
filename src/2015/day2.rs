use std::{
    result::Result,
    error::Error,
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
        format!("Day2 (2015): ({}, {})", self.part1(), self.part2())
    }
}


#[cfg(test)]
mod d215_testing {
    use aoc_lib::{Day, aw};

    fn new() -> super::Day {
        aw!(super::Day::new()).unwrap()
    }

    #[test]
    fn p1() {
        assert_eq!(1598415, new().part1());
    }

    #[test]
    fn p2() {
        assert_eq!(3812909, new().part2());
    }
}