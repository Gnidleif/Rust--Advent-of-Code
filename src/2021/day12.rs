use std::{
    result::Result,
    error::Error,
    time::Instant,
    collections::{
        HashMap,
        hash_map::{
            Entry,
            DefaultHasher,
        },
    },
    hash::{
        Hash,
        Hasher,
    },
};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Node {
    Start,
    End,
    Small(u64),
    Big(u64),
}

impl Node {
    fn new(s: &str) -> Self {
        if s == "start" {
            Node::Start
        }
        else if s == "end" {
            Node::End
        }
        else {
            let mut hs = DefaultHasher::new();
            s.hash(&mut hs);
            if s.chars().all(char::is_uppercase) {
                Node::Big(hs.finish())
            }
            else {
                Node::Small(hs.finish())
            }
        }
    }
}

pub struct Day {
    input: HashMap<Node, Vec<Node>>,
}

impl Day {
    #[allow(dead_code)]
    pub async fn new(run_sample: bool) -> Result<Self, Box<dyn Error>> {
        let content = aoc_lib::create_input(2021, 12, run_sample).await?;

        Ok(Day {
            input: content.lines()
            .map(|line| {
                let mut it = line.split("-");
                (Node::new(it.next().unwrap()), Node::new(it.next().unwrap()))
            })
            .fold(HashMap::new(), |mut acc: HashMap<Node, Vec<Node>>, (from, to): (Node, Node)| {
                match acc.entry(from) {
                    Entry::Vacant(e) => { e.insert(vec![to]); },
                    Entry::Occupied(mut e) => { e.get_mut().push(to); },
                };
        
                match acc.entry(to) {
                    Entry::Vacant(e) => { e.insert(vec![from]); },
                    Entry::Occupied(mut e) => { e.get_mut().push(from); },
                };
        
                acc
            }),
        })
    }

    fn traverse_caves(&self, node: Node, path: &mut Vec<Node>, visit_twice: bool) -> usize {
        path.push(node);
        let num_paths: usize = self.input.get(&node).unwrap().iter().copied()
            .map(|n| match n {
                    Node::Small(_) if !visit_twice || !path.contains(&n) => 
                        self.traverse_caves(n, path, visit_twice || path.contains(&n)),
                    Node::Big(_) => self.traverse_caves(n, path, visit_twice),
                    Node::End => 1,
                    _ => 0,
                }).sum();
        path.pop();

        num_paths
    }
}

impl aoc_lib::Day for Day {
    fn part1(&self) -> usize {
        self.traverse_caves(Node::Start, &mut vec![], true)
    }

    fn part2(&self) -> usize {
        self.traverse_caves(Node::Start, &mut vec![], false)
    }

    fn fmt_result(&self) -> String {
        let now1 = Instant::now();
        let p1 = self.part1();
        let elapsed1 = now1.elapsed().as_micros();
        let now2 = Instant::now();
        let p2 = self.part2();
        let elapsed2 = now2.elapsed().as_micros();
        format!("Day12 (2021): ({}: {}μs, {}: {}μs)", p1, elapsed1, p2, elapsed2)
    }
}


#[cfg(test)]
mod testing {
    use aoc_lib::{Day, aw};

    #[test]
    fn run() {
        let day = aw!(super::Day::new(false)).unwrap();
        assert_eq!(5228, day.part1());
        assert_eq!(131228, day.part2());
    }
}