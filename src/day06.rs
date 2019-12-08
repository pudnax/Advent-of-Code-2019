use std::io;
use std::ops::Deref;

use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let (graph1, graph2) = parse_input(input)?;
    let nconnections = graph1.nconnections("COM".to_string());
    let shortest_distance = graph2
        .shortest_distance("YOU".to_string(), "SAN".to_string())
        .ok_or_else(|| error!("Could not fild a path from us to Santa :("))?;
    let answer2 = shortest_distance - 2;
    Ok((nconnections.to_string(), answer2.to_string()))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Graph(HashMap<String, Vec<String>>);

impl Graph {
    fn shortest_distance(&self, a: String, b: String) -> Option<usize> {
        let mut queue: VecDeque<&str> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();

        let mut levels: HashMap<&str, usize> = HashMap::new();

        levels.insert(&a, 0);

        queue.push_back(&a);
        visited.insert(&a);

        while let Some(ref node) = queue.pop_front() {
            if let Some(children) = self.0.get(node.deref()) {
                for child in children {
                    if !visited.contains(child.deref()) {
                        let level = levels.get(node).unwrap() + 1;
                        levels.insert(child, level);

                        if child == &b {
                            return Some(level);
                        }

                        visited.insert(child);
                        queue.push_back(child);
                    }
                }
            }
        }

        None
    }

    fn nconnections(&self, start: String) -> usize {
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

        nconnections
    }
}

fn parse_input<R>(mut reader: R) -> Result<(Graph, Graph), Error>
where
    R: io::BufRead,
{
    let mut graph1 = HashMap::new();
    let mut graph2 = HashMap::new();
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

        graph1
            .entry(parent.clone())
            .or_insert_with(|| Vec::new())
            .push(child.clone());

        graph2
            .entry(parent.clone())
            .or_insert_with(|| Vec::new())
            .push(child.clone());

        graph2
            .entry(child)
            .or_insert_with(|| Vec::new())
            .push(parent);

        buffer.clear();
    }

    Ok((Graph(graph1), Graph(graph2)))
}
