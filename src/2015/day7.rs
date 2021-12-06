use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashMap,
};
use onig::Regex;

pub struct Day {
    input: Vec<Op>,
    map: HashMap<String, u16>,
}

#[derive(Debug, Clone)]
enum Op {
    Set(String, String),
    And(String, String, String),
    Or(String, String, String),
    Lshift(String, String, String),
    Rshift(String, String, String),
    Not(String, String),
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 7, run_sample).await?;

        let rgx = Regex::new(r"([A-Z]+)").unwrap();

        let (cmds, charges) = content.lines().map(|line| {
            let cmd: String = rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).collect();
            let splits: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
            let key: String = splits.iter().last().unwrap().to_string();
            (key, cmd, splits)
        }).fold((Vec::new(), HashMap::new()), |(mut v, mut m), (key, cmd, s)| {
            v.push(match &cmd[..] {
                "AND" => Op::And(s[0].clone(), s[2].clone(), key.clone()),
                "OR" => Op::Or(s[0].clone(), s[2].clone(), key.clone()),
                "LSHIFT" => Op::Lshift(s[0].clone(), s[2].clone(), key.clone()),
                "RSHIFT" => Op::Rshift(s[0].clone(), s[2].clone(), key.clone()),
                "NOT" => Op::Not(s[1].clone(), key.clone()),
                _ => Op::Set(s[0].clone(), key.clone()),
            });
            m.insert(key, 0);
            (v, m)
        });

        Ok(Day {
            input: cmds,
            map: charges,
        })
    }

    fn get_val(map: &HashMap<String, u16>, s: &String) -> u16 {
        if map.contains_key(s) {
            map[s]
        }
        else {
            s.parse().unwrap()
        }
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        let mut charges = self.map.clone();
        let mut cmds = self.input.clone();
        let mut len_check = 0;
        while len_check != cmds.len() {
            len_check = cmds.len();
            cmds.retain(|cmd| {
                let (key, val) = match cmd {
                    Op::Set(x, k) => {
                        let xv = Day::get_val(&charges, x);
                        if xv > 0 {
                            (k, xv)
                        } else {
                            (k, 0)
                        }
                    },
                    Op::And(x, y, k) => {
                        let (xv, yv) = (Day::get_val(&charges, x), Day::get_val(&charges, y));
                        if xv > 0 || yv > 0 {
                            (k, xv & yv)
                        } else {
                            (k, 0)
                        }
                    },
                    Op::Or(x, y, k) => {
                        let (xv, yv) = (Day::get_val(&charges, x), Day::get_val(&charges, y));
                        if xv > 0 || yv > 0 {
                            (k, xv | yv)
                        } else {
                            (k, 0)
                        }
                    },
                    Op::Lshift(x, y, k) => {
                        let (xv, yv) = (Day::get_val(&charges, x), Day::get_val(&charges, y));
                        if xv > 0 || yv > 0 {
                            (k, xv << yv)
                        } else {
                            (k, 0)
                        }
                    },
                    Op::Rshift(x, y, k) => {
                        let (xv, yv) = (Day::get_val(&charges, x), Day::get_val(&charges, y));
                        if xv > 0 || yv > 0 {
                            (k, xv >> yv)
                        } else {
                            (k, 0)
                        }
                    },
                    Op::Not(x, k) => {
                        let xv = Day::get_val(&charges, x);
                        if xv > 0 {
                            (k, !xv)
                        } else {
                            (k, 0)
                        }
                    },
                };
                if val > 0 {
                    charges.insert(key.to_string(), (val + charges[key]) % u16::MAX);
                }
                val == 0
            });
        }
        // charges["a"] as i32
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
        format!("Day7 (2015): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(3176, day.part1());
        assert_eq!(0, day.part2());
    }
}