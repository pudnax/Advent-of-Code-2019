use std::io;

use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};

type Ids = HashMap<String, usize>;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let (graph1, graph2, ids) = parse_input(input)?;
    let id_com = *ids.get("COM").ok_or_else(|| error!("COM node missing"))?;
    let nconnections = graph1.nconnections(id_com);

    let id_you = *ids.get("YOU").ok_or_else(|| error!("YOU node missing"))?;
    let id_san = *ids.get("SAN").ok_or_else(|| error!("SAN node missing"))?;
    let shortest_distance = graph2
        .shortest_distance(id_you, id_san)
        .ok_or_else(|| error!("Could not fild a path from us to Santa :("))?;
    let answer2 = shortest_distance - 2;
    Ok((nconnections.to_string(), answer2.to_string()))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Graph(HashMap<usize, Vec<usize>>);

impl std::ops::Deref for Graph {
    type Target = HashMap<usize, Vec<usize>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Graph {
    fn shortest_distance(&self, a: usize, b: usize) -> Option<usize> {
        let mut levels = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        levels.insert(a, 0);

        queue.push_back(a);
        visited.insert(a);

        while let Some(ref node) = queue.pop_front() {
            if let Some(children) = self.get(node) {
                for child in children {
                    if !visited.contains(child) {
                        let level = levels.get(node).unwrap() + 1;
                        levels.insert(*child, level);

                        if child == &b {
                            return Some(level);
                        }

                        visited.insert(*child);
                        queue.push_back(*child);
                    }
                }
            }
        }

        None
    }

    fn nconnections(&self, start: usize) -> usize {
        let mut levels = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let mut nconnections = 0;

        levels.insert(start, 0);

        queue.push_back(start);
        visited.insert(start);

        while let Some(ref node) = queue.pop_front() {
            if let Some(children) = self.get(node) {
                for child in children {
                    if !visited.contains(child) {
                        let level = levels.get(node).unwrap() + 1;
                        levels.insert(*child, level);

                        nconnections += level;

                        visited.insert(*child);
                        queue.push_back(*child);
                    }
                }
            }
        }

        nconnections
    }
}

fn parse_input<R>(mut reader: R) -> Result<(Graph, Graph, Ids), Error>
where
    R: io::BufRead,
{
    let mut graph1 = HashMap::new();
    let mut graph2 = HashMap::new();
    let mut buffer = String::new();
    let mut id = 0;
    let mut ids: HashMap<String, usize> = HashMap::new();
    while reader.read_line(&mut buffer)? > 0 {
        let line = buffer.trim();
        let mut iter = line.split(')').map(|s| s.trim().to_string());
        let parent = iter
            .next()
            .ok_or_else(|| error!("Unable to parse input line {}", line))?;

        let child = iter
            .next()
            .ok_or_else(|| error!("Unable to parse input line {}", line))?;

        let id_parent = *ids.entry(parent).or_insert_with(|| id);
        let id_child = *ids.entry(child).or_insert_with(|| id + 1);
        id += 2;

        graph1
            .entry(id_parent)
            .or_insert_with(|| Vec::new())
            .push(id_child);

        graph2
            .entry(id_parent)
            .or_insert_with(|| Vec::new())
            .push(id_child);

        graph2
            .entry(id_child)
            .or_insert_with(|| Vec::new())
            .push(id_parent);

        buffer.clear();
    }

    Ok((Graph(graph1), Graph(graph2), ids))
}
