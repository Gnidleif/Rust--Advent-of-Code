use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::HashMap,
};
use onig::Regex;

pub struct Day {
    input: Vec<BitCmd>,
    map: HashMap<String, u16>,
}

#[derive(Debug)]
enum BitCmd {
    Ignore,
    Set(String, String),
    And(String, String, String),
    Or(String, String, String),
    Lshift(String, String, String),
    Rshift(String, String, String),
    Not(String, String),
}

impl Day {
    #[allow(dead_code)]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2015, 7).await?;
        // let content = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let rgx = Regex::new(r"([A-Z]+)").unwrap();

        let (input, map) = content.lines().map(|line| {
            let cmd: String = rgx.find_iter(line).map(|pos| line[pos.0..pos.1].to_string()).collect();
            let splits: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
            let key: String = line.split_whitespace().last().unwrap().to_string();
            (key, cmd, splits)
        }).fold((Vec::new(), HashMap::new()), |(mut v, mut map), (key, cmd, s)| {
            map.insert(key.clone(), 0);
            v.push(match &cmd[..] {
                "AND" => BitCmd::And(s[0].clone(), s[2].clone(), s[4].clone()),
                "OR" => BitCmd::Or(s[0].clone(), s[2].clone(), s[4].clone()),
                "LSHIFT" => BitCmd::Lshift(s[0].clone(), s[2].clone(), s[4].clone()),
                "RSHIFT" => BitCmd::Rshift(s[0].clone(), s[2].clone(), s[4].clone()),
                "NOT" => BitCmd::Not(s[1].clone(), s[3].clone()),
                _ => match s[0].parse::<u16>() {
                        Ok(n) => { 
                            map.insert(key, n); 
                            BitCmd::Ignore
                        },
                        Err(_) => BitCmd::Set(s[0].clone(), s[2].clone()),
                    },
            });
            (v, map)
        });

        println!("{:?} {:?}", input, map);

        Ok(Day {
            input: input,
            map: map,
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
    fn part1(&self) -> i32 {
        let map = self.input.iter()
            .fold(self.map.clone(), |mut map, cmd| { 
                match cmd {
                    BitCmd::Ignore => {},
                    BitCmd::Set(x, key) => {
                        let xv = Day::get_val(&map, x);
                        map.insert(key.to_string(), xv);
                    }
                    BitCmd::And(x, y, key) => {
                        let xv = Day::get_val(&map, x);
                        let yv = Day::get_val(&map, y);
                        map.insert(key.to_string(), xv & yv);
                    },
                    BitCmd::Or(x, y, key) => {
                        let xv = Day::get_val(&map, x);
                        let yv = Day::get_val(&map, y);
                        map.insert(key.to_string(), xv | yv);
                    },
                    BitCmd::Lshift(x, y, key) => {
                        let xv = Day::get_val(&map, x);
                        let yv = Day::get_val(&map, y);
                        map.insert(key.to_string(), xv << yv);
                    },
                    BitCmd::Rshift(x, y, key) => {
                        let xv = Day::get_val(&map, x);
                        let yv = Day::get_val(&map, y);
                        map.insert(key.to_string(), xv >> yv);
                    },
                    BitCmd::Not(x, key) => {
                        let xv = Day::get_val(&map, x);
                        map.insert(key.to_string(), !xv);
                    }
                }
                map
            });

        println!("{:?}", map);

        0
    }

    fn part2(&self) -> i32 {
        0
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_millis();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_millis();
        format!("Day7 (2015): ({}: {}ms, {}: {}ms)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new()).unwrap();
        assert_eq!(3176, day.part1());
        assert_eq!(0, day.part2());
    }
}