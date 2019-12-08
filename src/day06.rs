use std::io;
use std::ops::Deref;

use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let graph = parse_input(input)?;
    let nconnections = graph.nconnections("COM".to_string())?;
    println!("{:?}", nconnections);
    Ok(("foo".to_string(), "bar".to_string()))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Graph(HashMap<String, Vec<String>>);

impl Graph {
    fn nconnections(&self, start: String) -> Result<usize, Error> {
        let mut queue: VecDeque<&str> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();

        let mut nconnections = 0;

        let mut levels: HashMap<&str, usize> = HashMap::new();

        levels.insert(&start, 0);

        queue.push_back(&start);
        visited.insert(&start);

        while let Some(ref node) = queue.pop_front() {
            if let Some(children) = self.0.get(node.deref()) {
                for child in children {
                    if !visited.contains(child.deref()) {
                        let level = levels.get(node).unwrap() + 1;
                        levels.insert(child, level);

                        nconnections += level;

                        visited.insert(child);
                        queue.push_back(child);
                    }
                }
            }
        }

        Ok(nconnections)
    }
}

fn parse_input<R>(mut reader: R) -> Result<Graph, Error>
where
    R: io::BufRead,
{
    let mut map = HashMap::new();
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        let line = buffer.trim();
        let mut iter = line.split(')').map(|s| s.trim().to_string());
        let parent = iter
            .next()
            .ok_or_else(|| error!("Unable to parse input line {}", line))?;

        let child = iter
            .next()
            .ok_or_else(|| error!("Unable to parse input line {}", line))?;

        map.entry(parent).or_insert_with(|| Vec::new()).push(child);

        buffer.clear();
    }

    Ok(Graph(map))
}
