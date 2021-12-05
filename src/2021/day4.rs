use std::{
    result::Result,
    error::Error,
    time::Instant,
};
use itertools::{
    Itertools,
    FoldWhile::{
        Continue, 
        Done,
    },
};

pub struct Day {
    input: Vec<Board>,
    numbers: Vec<i32>,
}

#[derive(Clone, Debug)]
struct Cell {
    num: i32,
    done: bool, 
}

#[derive(Clone, Debug)]
struct Board {
    width: usize,
    height: usize,
    matrix: Vec<Cell>,
}

impl Board {
    fn new(numbers: &[String]) -> Self {
        Board {
            width: 5,
            height: 5,
            matrix: numbers.iter()
                .fold(Vec::new(), |mut acc, line| {
                    line.split_whitespace().map(|s| Cell {
                        num: s.parse::<i32>().unwrap(),
                        done: false,
                    }).for_each(|x| acc.push(x));
                    acc
                }),
        }
    }

    fn check_slice(v: &[Cell]) -> bool {
        v.iter().filter(|x| x.done).count() == v.len()
    }

    fn check_rows(&self) -> i32 {
        (0..self.matrix.len()).step_by(self.width).fold_while(0, |_, i| {
            let row = &self.matrix[i..i+self.width];
            if Board::check_slice(row) {
                Done(self.matrix.iter().filter(|c| !c.done).map(|c| c.num).sum())
            }
            else {
                Continue(0)
            }
        }).into_inner()
    }

    fn check_cols(&self) -> i32 {
        (0..self.matrix.len() / self.height).fold_while(0, |_, i| {
            let idx: Vec<usize> = (0..self.matrix.len()).step_by(self.height).map(|j| i + j).collect();
            let column: &[Cell] = &idx.iter().fold(Vec::new(), |mut acc, j| {
                acc.push(self.matrix[*j].clone());
                acc
            })[..];
            if Board::check_slice(column) {
                Done(self.matrix.iter().filter(|c| !c.done).map(|c| c.num).sum())
            }
            else {
                Continue(0)
            }
        }).into_inner()
    }

    fn index_of(&self, num: &i32) -> i32 {
        match self.matrix.iter().enumerate().find(|(_, c)| c.num == *num) {
            Some((i, _)) => i as i32,
            None => -1,
        }
    }
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 4, run_sample).await?;

        let mut lines = content.lines();
        let numbers: Vec<i32> = lines.next().unwrap().split(",").map(|num| num.parse::<i32>().unwrap()).collect();
        let boards: Vec<String> = lines.filter(|line| line.len() > 0).map(|line| line.to_string()).collect();

        let input = boards.iter().enumerate().step_by(5).fold(Vec::new(), |mut acc: Vec<Board>, (i, _)| {
            acc.push(Board::new(&boards[i..i+5]));
            acc
        });

        Ok(Day {
            input: input,
            numbers: numbers,
        })
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> i32 {
        let mut boards = self.input.clone();
        for num in self.numbers.iter() {
            for i in 0..boards.len() {
                let j = boards[i].index_of(&num);
                if j == -1 {
                    continue;
                }
                boards[i].matrix[j as usize].done = true;
                let s = boards[i].check_rows();
                if s > 0 {
                    return s * num;
                }
                let s = boards[i].check_cols();
                if s > 0 {
                    return s * num;
                }
            }
        }
        0
    }

    fn part2(&self) -> i32 {
        let mut result = 0;
        let mut windices = vec![false; self.input.len()];
        let mut boards = self.input.clone();
        for num in self.numbers.iter() {
            for i in 0..boards.len() {
                if windices[i] {
                    continue;
                }
                let j = boards[i].index_of(&num);
                if j == -1 {
                    continue;
                }
                boards[i].matrix[j as usize].done = true;
                let s = boards[i].check_rows();
                if s > 0 {
                    result = s * num;
                    windices[i] = true;
                    continue;
                }
                let s = boards[i].check_cols();
                if s > 0 {
                    result = s * num;
                    windices[i] = true;
                }
            }
            if windices.iter().filter(|x| !**x).count() == 0 {
                break;
            }
        }
        result
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day4 (2021): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(87456, day.part1());
        assert_eq!(15561, day.part2());
    }
}